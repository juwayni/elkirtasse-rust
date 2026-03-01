pub mod models;
pub mod parser;
pub mod text_utils;
pub mod search;

use crate::models::{Category, Page, Chapter, SearchResult, Annotation};
use crate::parser::{parse_group_xml, parse_book_xml, parse_title_xml};
use crate::search::SearchEngine;
use tauri::{command, AppHandle, Manager, State};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AppState {
    pub search_engine: Mutex<Option<SearchEngine>>,
    pub annotations: Mutex<Vec<Annotation>>,
}

fn resolve_data_path(app: &AppHandle) -> PathBuf {
    #[cfg(debug_assertions)]
    {
        PathBuf::from("..").join("usr").join("share").join("elkirtasse").join("data")
    }
    #[cfg(not(debug_assertions))]
    {
        app.path().resource_dir().unwrap_or_default().join("data")
    }
}

fn get_annotations_file(app: &AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap_or_default().join("annotations.json")
}

#[command]
pub async fn get_library(app: AppHandle) -> Result<Vec<Category>, String> {
    let group_xml_path = resolve_data_path(&app).join("group.xml");
    let content = fs::read_to_string(group_xml_path).map_err(|e| e.to_string())?;
    parse_group_xml(&content).map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
pub struct BookContent {
    pub pages: Vec<Page>,
    pub chapters: Vec<Chapter>,
}

#[command]
pub async fn get_book_content(app: AppHandle, book_id: String) -> Result<BookContent, String> {
    let books_dir = resolve_data_path(&app).parent().unwrap_or(Path::new(".")).join("books");
    let book_xml_path = books_dir.join(&book_id).join("book.xml");
    let title_xml_path = books_dir.join(&book_id).join("title.xml");

    let pages = if book_xml_path.exists() {
        let content = fs::read_to_string(book_xml_path).map_err(|e| e.to_string())?;
        parse_book_xml(&content).map_err(|e| e.to_string())?
    } else {
        vec![]
    };

    let chapters = if title_xml_path.exists() {
        let content = fs::read_to_string(title_xml_path).map_err(|e| e.to_string())?;
        parse_title_xml(&content).map_err(|e| e.to_string())?
    } else {
        vec![]
    };

    Ok(BookContent { pages, chapters })
}

#[command]
pub async fn search_books(state: State<'_, AppState>, query: String, book_filter: Option<Vec<String>>) -> Result<Vec<SearchResult>, String> {
    let engine_lock = state.search_engine.lock().map_err(|_| "Failed to lock search engine")?;
    if let Some(engine) = engine_lock.as_ref() {
        engine.search(&query, book_filter).map_err(|e| e.to_string())
    } else {
        Err("Search engine not initialized".to_string())
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
                let content = get_book_content(app.clone(), book.id.clone()).await.unwrap_or(BookContent { pages: vec![], chapters: vec![] });
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

    // Persist to file
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
            app.manage(AppState {
                search_engine: Mutex::new(None),
                annotations: Mutex::new(annotations),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_library,
            get_book_content,
            search_books,
            start_indexing,
            add_annotation,
            get_annotations
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
