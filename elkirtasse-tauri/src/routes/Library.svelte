<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { fade, slide } from 'svelte/transition';
  import BookEditor from "./BookEditor.svelte";
  import BookCover from "./BookCover.svelte";

  export let onOpenBook: (bookId: string, title: string) => void;

  interface Book {
    id: string;
    name: string;
    author: string;
    betaka?: string;
    progress?: number;
  }

  interface Category {
    id: string;
    name: string;
    sub_categories: Category[];
    books: Book[];
  }

  let categories: Category[] = [];
  let selectedCategory: Category | null = null;
  let expandedCats: Set<string> = new Set();
  let editingBook: Book | null = null;

  async function batchImport() {
    const dirPath = prompt("Enter directory path to scan for books:");
    if (!dirPath) return;
    try {
      const count: number = await invoke("batch_import", { dirPath, categoryId: selectedCategory?.id || "root" });
      await loadLibrary();
      alert(`Successfully imported ${count} books!`);
    } catch (e) {
      alert("Error: " + e);
    }
  }

  async function importBook() {
    const sourcePath = prompt("Enter full path to book file:");
    if (!sourcePath) return;
    try {
      await invoke("import_book", { sourcePath, categoryId: selectedCategory?.id || "root" });
      await loadLibrary();
      alert("Book imported!");
    } catch (e) {
      alert("Error: " + e);
    }
  }

  async function loadLibrary() {
    try {
      categories = await invoke("get_library");
    } catch (e) {
      console.error("Failed to load library", e);
    }
  }

  function toggleCat(id: string) {
    if (expandedCats.has(id)) {
      expandedCats.delete(id);
    } else {
      expandedCats.add(id);
    }
    expandedCats = expandedCats;
  }

  async function deleteBook(bookId: string) {
    if (!confirm("Are you sure you want to delete this book? This cannot be undone.")) return;
    try {
      await invoke("delete_book", { bookId });
      loadLibrary();
    } catch (e) {
      console.error("Failed to delete book", e);
    }
  }

  onMount(loadLibrary);
</script>

<div class="flex h-full bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100 transition-colors duration-300">
  <aside class="w-80 border-r border-gray-100 dark:border-gray-800 overflow-y-auto p-6 bg-gray-50/50 dark:bg-gray-900/50 backdrop-blur-xl">
    <div class="flex items-center justify-between mb-8">
      <h2 class="text-xs font-black tracking-widest text-gray-400 uppercase">Categories</h2>
      <div class="flex gap-1">
        <button
          class="text-blue-600 p-2 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-full transition-colors"
          title="Batch Import"
          on:click={batchImport}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
        </button>
        <button
          class="text-blue-600 p-2 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-full transition-colors"
          title="Import Book"
          on:click={importBook}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
        </button>
      </div>
    </div>

    <ul class="space-y-1">
      {#each categories as cat}
        <li>
          <div
            role="button"
            tabindex="0"
            class="flex items-center justify-between p-3 rounded-xl cursor-pointer transition-all hover:bg-white dark:hover:bg-gray-800 shadow-sm hover:shadow {selectedCategory?.id === cat.id ? 'bg-white dark:bg-gray-800 border-l-4 border-blue-600' : 'border-l-4 border-transparent'}"
            on:click={() => { selectedCategory = cat; toggleCat(cat.id); }}
            on:keydown={(e) => e.key === 'Enter' && (selectedCategory = cat, toggleCat(cat.id))}
          >
            <span class="font-bold text-sm">{cat.name}</span>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 transition-transform {expandedCats.has(cat.id) ? 'rotate-90' : ''}" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </div>

          {#if expandedCats.has(cat.id) && cat.sub_categories.length > 0}
            <ul class="ml-4 mt-1 space-y-1" transition:slide>
              {#each cat.sub_categories as sub}
                <li>
                  <button
                    class="w-full text-left p-2.5 pl-4 rounded-lg text-sm cursor-pointer transition-colors {selectedCategory?.id === sub.id ? 'text-blue-600 dark:text-blue-400 font-bold bg-blue-50 dark:bg-blue-900/20' : 'text-gray-500 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-800'}"
                    on:click={() => selectedCategory = sub}
                  >
                    {sub.name}
                  </button>
                </li>
              {/each}
            </ul>
          {/if}
        </li>
      {/each}
    </ul>
  </aside>

  <main class="flex-1 p-8 lg:p-12 overflow-y-auto">
    {#if selectedCategory}
      <div class="max-w-6xl mx-auto" in:fade>
        <div class="flex items-end justify-between mb-10">
          <div>
            <h2 class="text-4xl font-black text-gray-900 dark:text-white mb-2">{selectedCategory.name}</h2>
            <p class="text-gray-500 font-medium">{selectedCategory.books.length} volumes available</p>
          </div>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 gap-8">
          {#each selectedCategory.books as book}
            <div class="group flex flex-col h-full bg-white dark:bg-gray-900 p-6 rounded-[2.5rem] shadow-sm border border-gray-100 dark:border-gray-800 hover:border-blue-500/30 transition-all hover:shadow-2xl">
              <div class="mb-6 relative">
                 <BookCover title={book.name} author={book.author} progress={book.progress} />

                 <div class="absolute top-4 right-4 flex flex-col gap-2 opacity-0 group-hover:opacity-100 transition-all translate-x-4 group-hover:translate-x-0">
                    <button
                      class="p-2.5 bg-white/90 dark:bg-gray-900/90 backdrop-blur shadow-xl rounded-2xl text-gray-500 hover:text-blue-600 transition-colors"
                      on:click={() => editingBook = book}
                      aria-label="Edit Metadata"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
                      </svg>
                    </button>
                    <button
                      class="p-2.5 bg-white/90 dark:bg-gray-900/90 backdrop-blur shadow-xl rounded-2xl text-gray-500 hover:text-red-600 transition-colors"
                      on:click={() => deleteBook(book.id)}
                      aria-label="Delete Book"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                      </svg>
                    </button>
                 </div>
              </div>

              <h3 class="font-black text-xl mb-1 text-gray-900 dark:text-white leading-tight" dir="rtl">{book.name}</h3>
              <p class="text-gray-400 text-xs font-bold uppercase tracking-widest mb-6">{book.author}</p>

              <div class="mt-auto pt-6 border-t border-gray-50 dark:border-gray-800 flex items-center justify-between">
                <span class="text-xs font-bold text-gray-400 uppercase tracking-widest">Digital Copy</span>
                <button
                  class="bg-gray-900 dark:bg-white text-white dark:text-gray-900 px-6 py-2.5 rounded-xl text-sm font-black transition-all hover:bg-blue-600 hover:text-white group-hover:shadow-lg"
                  on:click={() => onOpenBook(book.id, book.name)}
                >
                  READ →
                </button>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {:else}
      <div class="flex flex-col items-center justify-center h-full text-gray-300 dark:text-gray-700" in:fade>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-48 w-48 mb-8 opacity-20" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
        </svg>
        <p class="text-3xl font-thin tracking-tighter">Explore the depths of knowledge</p>
        <p class="mt-4 text-sm font-bold uppercase tracking-[0.3em] text-blue-600/40">Select a category to begin</p>
      </div>
    {/if}
  </main>
</div>

{#if editingBook}
  <BookEditor
    book={editingBook}
    onCancel={() => editingBook = null}
    onSave={() => { editingBook = null; loadLibrary(); }}
  />
{/if}
