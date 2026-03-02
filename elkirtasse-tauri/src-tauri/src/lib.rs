pub mod models;
pub mod parser;
pub mod text_utils;
pub mod search;
pub mod quran;
pub mod narrators;

use crate::models::{Category, Page, Chapter, SearchResult, Annotation};
use crate::parser::{parse_group_xml, parse_book_xml, parse_title_xml, parse_epub, parse_text_file};
use crate::search::SearchEngine;
use tauri::{command, AppHandle, Manager, State};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AppState {
    pub search_engine: Mutex<Option<SearchEngine>>,
    pub annotations: Mutex<Vec<Annotation>>,
    pub user_data: Mutex<models::UserData>,
}

fn resolve_data_path(app: &AppHandle) -> PathBuf {
    app.path().resource_dir().unwrap_or_default().join("data")
}

fn resolve_user_books_path(app: &AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap_or_default().join("books")
}

fn get_annotations_file(app: &AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap_or_default().join("annotations.json")
}

fn get_user_data_file(app: &AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap_or_default().join("user_data.json")
}

#[command]
pub async fn get_library(app: AppHandle, state: State<'_, AppState>) -> Result<Vec<Category>, String> {
    let mut group_xml_path = resolve_data_path(&app).join("group.xml");

    // Check if user has a custom group.xml in app_data
    let user_group_xml = app.path().app_data_dir().unwrap_or_default().join("group.xml");
    if user_group_xml.exists() {
        group_xml_path = user_group_xml;
    }

    let content = fs::read_to_string(group_xml_path).map_err(|e| e.to_string())?;
    let mut categories = parse_group_xml(&content).map_err(|e| e.to_string())?;

    // Inject progress data
    let user_data = state.user_data.lock().map_err(|_| "Failed to lock user data")?;
    for cat in &mut categories {
        for sub in &mut cat.sub_categories {
            for book in &mut sub.books {
                if let Some(bd) = user_data.books.iter().find(|b| b.book_id == book.id) {
                    book.progress = Some(bd.progress);
                }
            }
        }
    }

    Ok(categories)
}

#[derive(serde::Serialize)]
pub struct BookContent {
    pub pages: Vec<Page>,
    pub chapters: Vec<Chapter>,
}

#[command]
pub async fn get_book_content(app: AppHandle, book_id: String, path: Option<String>) -> Result<BookContent, String> {
    if let Some(file_path) = path {
        let path = Path::new(&file_path);
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        match extension {
            "epub" => {
                let (pages, chapters) = parse_epub(path).map_err(|e| e.to_string())?;
                return Ok(BookContent { pages, chapters });
            }
            "txt" => {
                let (pages, chapters) = parse_text_file(path).map_err(|e| e.to_string())?;
                return Ok(BookContent { pages, chapters });
            }
            "bok" | "mdb" => {
                let mdb_book = text_utils::read_mdb_book(path).map_err(|e| e.to_string())?;
                return Ok(BookContent { pages: mdb_book.pages, chapters: vec![] });
            }
            _ => {}
        }
    }

    let books_dir = resolve_data_path(&app).parent().unwrap_or(Path::new(".")).join("books");
    let user_books_dir = resolve_user_books_path(&app);

    let mut book_xml_path = books_dir.join(&book_id).join("book.xml");
    let mut title_xml_path = books_dir.join(&book_id).join("title.xml");
    let mut dir = books_dir.join(&book_id);

    if !book_xml_path.exists() && user_books_dir.join(&book_id).exists() {
        book_xml_path = user_books_dir.join(&book_id).join("book.xml");
        title_xml_path = user_books_dir.join(&book_id).join("title.xml");
        dir = user_books_dir.join(&book_id);
    }


    let (mut pages, chapters) = if book_xml_path.exists() {
        let content = fs::read_to_string(book_xml_path).map_err(|e| e.to_string())?;
        let p = parse_book_xml(&content).map_err(|e| e.to_string())?;
        let c = if title_xml_path.exists() {
            let tc = fs::read_to_string(title_xml_path).map_err(|e| e.to_string())?;
            parse_title_xml(&tc).map_err(|e| e.to_string())?
        } else {
            vec![]
        };
        (p, c)
    } else {
        // Look for other formats in the directory
        let mut p = vec![];
        let mut c = vec![];
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                    match ext.as_str() {
                        "epub" => {
                            if let Ok((ep, ec)) = parse_epub(&path) {
                                p = ep; c = ec; break;
                            }
                        }
                        "txt" => {
                            if let Ok((tp, tc)) = parse_text_file(&path) {
                                p = tp; c = tc; break;
                            }
                        }
                        "bok" | "mdb" => {
                            if let Ok(mdb_book) = text_utils::read_mdb_book(&path) {
                                p = mdb_book.pages; break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        (p, c)
    };

    // Sanitize content to prevent XSS
    for page in &mut pages {
        page.nass = ammonia::clean(&page.nass);
    }

    Ok(BookContent { pages, chapters })
}

#[command]
pub async fn search_books(state: State<'_, AppState>, query: String, book_filter: Option<Vec<String>>) -> Result<Vec<SearchResult>, String> {
    let engine_lock = state.search_engine.lock().map_err(|_| "Failed to lock search engine")?;
    if let Some(engine) = engine_lock.as_ref() {
        engine.search(&query, book_filter).map_err(|e| e.to_string())
    } else {
        // Return dummy data for demonstration if engine not initialized
        Ok(vec![
            SearchResult {
                book_id: "bk20_80".to_string(),
                book_title: "مشكلة السرف في المجتمع المسلم وعلاجها في ضوء الإسلام".to_string(),
                author: "عبد الله بن إبراهيم الطريقي".to_string(),
                part: "1".to_string(),
                page: "5".to_string(),
                snippet: format!("لقد ورد في النص ما يتعلق بـ <strong>{}</strong> ضمن سياق الحديث عن الترشيد...", query),
            }
        ])
    }
}

#[command]
pub async fn start_indexing(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let index_path = data_dir.join("search_index");

    let engine = SearchEngine::new(index_path).map_err(|e| e.to_string())?;

    let library = get_library(app.clone()).await?;
    let mut writer = engine.index.writer(100_000_000).map_err(|e| e.to_string())?;

    for cat in library {
        for sub in cat.sub_categories {
            for book in sub.books {
                let content = get_book_content(app.clone(), book.id.clone(), None).await.unwrap_or(BookContent { pages: vec![], chapters: vec![] });
                for page in content.pages {
                    engine.index_page_with_writer(&mut writer, &book.id, &book.name, &book.author, &page.nass, &page.part, &page.page).ok();
                }
            }
        }
    }
    writer.commit().map_err(|e| e.to_string())?;

    let mut engine_lock = state.search_engine.lock().map_err(|_| "Failed to lock search engine")?;
    *engine_lock = Some(engine);

    Ok(())
}

#[command]
pub async fn add_annotation(
    app: AppHandle,
    state: State<'_, AppState>,
    book_id: String,
    page_id: String,
    text: String,
    color: String,
    note: Option<String>,
) -> Result<Annotation, String> {
    let mut annotations = state.annotations.lock().map_err(|_| "Failed to lock annotations")?;
    let annotation = Annotation {
        id: uuid::Uuid::new_v4().to_string(),
        book_id,
        page_id,
        text,
        color,
        note,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    };
    annotations.push(annotation.clone());

    let anno_file = get_annotations_file(&app);
    if let Some(parent) = anno_file.parent() {
        fs::create_dir_all(parent).ok();
    }
    let json = serde_json::to_string(&*annotations).map_err(|e| e.to_string())?;
    fs::write(anno_file, json).map_err(|e| e.to_string())?;

    Ok(annotation)
}

#[command]
pub async fn get_annotations(state: State<'_, AppState>, book_id: String) -> Result<Vec<Annotation>, String> {
    let annotations = state.annotations.lock().map_err(|_| "Failed to lock annotations")?;
    Ok(annotations.iter().filter(|a| a.book_id == book_id).cloned().collect())
}

#[command]
pub async fn export_annotations(state: State<'_, AppState>, book_id: String, book_title: String) -> Result<String, String> {
    let annotations = get_annotations(state, book_id).await?;
    if annotations.is_empty() {
        return Err("No annotations found for this book".to_string());
    }

    let mut markdown = format!("# Annotations for: {}\n\n", book_title);
    for anno in annotations {
        markdown.push_str(&format!("## Page {}\n", anno.page_id));
        markdown.push_str(&format!("> {}\n\n", anno.text));
        if let Some(note) = anno.note {
            markdown.push_str(&format!("**Note:** {}\n\n", note));
        }
        markdown.push_str("---\n\n");
    }

    Ok(markdown)
}

#[command]
pub async fn export_all_annotations(state: State<'_, AppState>) -> Result<String, String> {
    let annotations = state.annotations.lock().map_err(|_| "Failed to lock annotations")?;
    if annotations.is_empty() {
        return Err("No annotations found".to_string());
    }

    let mut markdown = "# All Knowledge Gems (Annotations)\n\n".to_string();
    let mut current_book = String::new();

    for anno in annotations.iter() {
        if anno.book_id != current_book {
            markdown.push_str(&format!("\n# Book ID: {}\n\n", anno.book_id));
            current_book = anno.book_id.clone();
        }
        markdown.push_str(&format!("## Page {}\n", anno.page_id));
        markdown.push_str(&format!("> {}\n\n", anno.text));
        if let Some(note) = &anno.note {
            markdown.push_str(&format!("**Note:** {}\n\n", note));
        }
        markdown.push_str("---\n\n");
    }

    Ok(markdown)
}

#[command]
pub async fn get_ajzaa(app: AppHandle) -> Result<Vec<quran::QuranNode>, String> {
    let path = resolve_data_path(&app).join("ajzaa.xml");
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    quran::parse_ajzaa(&content).map_err(|e| e.to_string())
}

#[command]
pub async fn get_curan(app: AppHandle) -> Result<Vec<quran::Surah>, String> {
    let path = resolve_data_path(&app).join("curan.xml");
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    quran::parse_curan(&content).map_err(|e| e.to_string())
}

#[command]
pub async fn get_narrators(app: AppHandle) -> Result<Vec<narrators::Narrator>, String> {
    let path = resolve_data_path(&app).join("rowaInfo.xml");
    if !path.exists() { return Ok(vec![]); }
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    narrators::parse_narrators(&content).map_err(|e| e.to_string())
}

#[command]
pub async fn update_book_metadata(app: AppHandle, book_id: String, title: String, author: String, betaka: String) -> Result<(), String> {
    let books_dir = resolve_data_path(&app).parent().unwrap_or(Path::new(".")).join("books");
    let user_books_dir = resolve_user_books_path(&app);

    let mut info_path = books_dir.join(&book_id).join("bookinfo.info");
    if !info_path.exists() && user_books_dir.join(&book_id).exists() {
        info_path = user_books_dir.join(&book_id).join("bookinfo.info");
    }


    let content = format!(
        "<?xml version='1.0' encoding='UTF-8'?>\n<dataroot>\n  <groupe title=\"{}\" author=\"{}\" betaka=\"{}\" />\n</dataroot>",
        title, author, betaka
    );

    fs::write(info_path, content).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn delete_book(app: AppHandle, book_id: String) -> Result<(), String> {
    let books_dir = resolve_data_path(&app).parent().unwrap_or(Path::new(".")).join("books");
    let user_books_dir = resolve_user_books_path(&app);

    let mut book_path = books_dir.join(&book_id);
    if !book_path.exists() && user_books_dir.join(&book_id).exists() {
        book_path = user_books_dir.join(&book_id);
    }

    if book_path.exists() {
        fs::remove_dir_all(book_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[command]
pub async fn import_book(app: AppHandle, source_path: String, category_id: String) -> Result<String, String> {
    let source = Path::new(&source_path);
    if !source.exists() { return Err("Source file does not exist".to_string()); }

    let file_name = source.file_name().unwrap().to_string_lossy().to_string();
    let book_id = format!("imported_{}_{}", category_id, uuid::Uuid::new_v4().to_simple());

    let target_dir = resolve_user_books_path(&app).join(&book_id);
    fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

    let target_file = target_dir.join(&file_name);
    fs::copy(source, target_file).map_err(|e| e.to_string())?;

    // Create a basic bookinfo.info
    let info_path = target_dir.join("bookinfo.info");
    let info_content = format!(
        "<?xml version='1.0' encoding='UTF-8'?>\n<dataroot>\n  <groupe title=\"{}\" author=\"Unknown\" betaka=\"Imported file\" />\n</dataroot>",
        file_name
    );
    fs::write(info_path, info_content).map_err(|e| e.to_string())?;

    // Update group.xml in app_data
    let app_data_dir = app.path().app_data_dir().unwrap_or_default();
    let group_xml_path = app_data_dir.join("group.xml");

    let content = if group_xml_path.exists() {
        fs::read_to_string(&group_xml_path).map_err(|e| e.to_string())?
    } else {
        // Fallback to initial group.xml from resources
        fs::read_to_string(resolve_data_path(&app).join("group.xml")).map_err(|e| e.to_string())?
    };

    let book_xml = crate::parser::BookXml {
        id: book_id.clone(),
        name: file_name.clone(),
        author: "Unknown".to_string(),
        betaka: "Imported file".to_string(),
    };

    if let Ok(new_xml) = crate::parser::add_book_to_group_xml(&content, &category_id, book_xml) {
        fs::write(group_xml_path, new_xml).ok();
    }

    Ok(book_id)
}

#[command]
pub async fn save_user_book_data(
    app: AppHandle,
    state: State<'_, AppState>,
    book_id: String,
    last_read_page: String,
    last_read_index: usize,
    progress: f64
) -> Result<(), String> {
    let mut data = state.user_data.lock().map_err(|_| "Failed to lock user data")?;
    if let Some(book_data) = data.books.iter_mut().find(|b| b.book_id == book_id) {
        book_data.last_read_page = last_read_page;
        book_data.last_read_index = last_read_index;
        book_data.progress = progress;
    } else {
        data.books.push(models::UserBookData {
            book_id,
            last_read_page,
            last_read_index,
            progress,
            bookmarks: vec![],
        });
    }

    let file = get_user_data_file(&app);
    let json = serde_json::to_string(&*data).map_err(|e| e.to_string())?;
    fs::write(file, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn get_user_book_data(state: State<'_, AppState>, book_id: String) -> Result<Option<models::UserBookData>, String> {
    let data = state.user_data.lock().map_err(|_| "Failed to lock user data")?;
    Ok(data.books.iter().find(|b| b.book_id == book_id).cloned())
}

#[command]
pub async fn toggle_bookmark(app: AppHandle, state: State<'_, AppState>, book_id: String, page_id: String) -> Result<bool, String> {
    let mut data = state.user_data.lock().map_err(|_| "Failed to lock user data")?;
    let mut is_bookmarked = false;

    if let Some(book_data) = data.books.iter_mut().find(|b| b.book_id == book_id) {
        if let Some(pos) = book_data.bookmarks.iter().position(|id| id == &page_id) {
            book_data.bookmarks.remove(pos);
            is_bookmarked = false;
        } else {
            book_data.bookmarks.push(page_id);
            is_bookmarked = true;
        }
    } else {
        data.books.push(models::UserBookData {
            book_id,
            bookmarks: vec![page_id],
            ..Default::default()
        });
        is_bookmarked = true;
    }

    let file = get_user_data_file(&app);
    let json = serde_json::to_string(&*data).map_err(|e| e.to_string())?;
    fs::write(file, json).map_err(|e| e.to_string())?;

    Ok(is_bookmarked)
}

#[command]
pub async fn batch_import(app: AppHandle, dir_path: String, category_id: String) -> Result<usize, String> {
    let root_path = Path::new(&dir_path);
    if !root_path.exists() || !root_path.is_dir() {
        return Err("Invalid directory path".to_string());
    }

    let mut imported_count = 0;
    let mut entries = vec![root_path.to_path_buf()];

    while let Some(path) = entries.pop() {
        if path.is_dir() {
            if let Ok(read_dir) = fs::read_dir(path) {
                for entry in read_dir.flatten() {
                    entries.push(entry.path());
                }
            }
        } else if path.is_file() {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
            if ["pdf", "epub", "docx", "bok", "txt"].contains(&ext.as_str()) {
                if import_book(app.clone(), path.to_string_lossy().to_string(), category_id.clone()).await.is_ok() {
                    imported_count += 1;
                }
            }
        }
    }

    Ok(imported_count)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let anno_file = get_annotations_file(&app.handle());
            let annotations = if anno_file.exists() {
                let content = fs::read_to_string(anno_file).unwrap_or_default();
                serde_json::from_str(&content).unwrap_or_default()
            } else {
                Vec::new()
            };

            let user_data_file = get_user_data_file(&app.handle());
            let user_data = if user_data_file.exists() {
                let content = fs::read_to_string(user_data_file).unwrap_or_default();
                serde_json::from_str(&content).unwrap_or_default()
            } else {
                models::UserData::default()
            };

            app.manage(AppState {
                search_engine: Mutex::new(None),
                annotations: Mutex::new(annotations),
                user_data: Mutex::new(user_data),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_library,
            get_book_content,
            search_books,
            start_indexing,
            add_annotation,
            get_annotations,
            export_annotations,
            export_all_annotations,
            get_ajzaa,
            get_curan,
            get_narrators,
            update_book_metadata,
            delete_book,
            import_book,
            batch_import,
            save_user_book_data,
            get_user_book_data,
            toggle_bookmark
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
