<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { fade, slide } from 'svelte/transition';

  interface Book {
    id: string;
    name: string;
    author: string;
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

  onMount(loadLibrary);
</script>

<div class="flex h-full bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100 transition-colors duration-300">
  <aside class="w-80 border-r border-gray-100 dark:border-gray-800 overflow-y-auto p-6 bg-gray-50/50 dark:bg-gray-900/50 backdrop-blur-xl">
    <div class="flex items-center justify-between mb-8">
      <h2 class="text-xs font-black tracking-widest text-gray-400 uppercase">Categories</h2>
      <button class="text-blue-600 text-xs font-bold hover:underline">Collapse All</button>
    </div>

    <ul class="space-y-1">
      {#each categories as cat}
        <li>
          <div
            class="flex items-center justify-between p-3 rounded-xl cursor-pointer transition-all hover:bg-white dark:hover:bg-gray-800 shadow-sm hover:shadow {selectedCategory?.id === cat.id ? 'bg-white dark:bg-gray-800 border-l-4 border-blue-600' : 'border-l-4 border-transparent'}"
            on:click={() => { selectedCategory = cat; toggleCat(cat.id); }}
          >
            <span class="font-bold text-sm">{cat.name}</span>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 transition-transform {expandedCats.has(cat.id) ? 'rotate-90' : ''}" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </div>

          {#if expandedCats.has(cat.id) && cat.sub_categories.length > 0}
            <ul class="ml-4 mt-1 space-y-1" transition:slide>
              {#each cat.sub_categories as sub}
                <li
                  class="p-2.5 pl-4 rounded-lg text-sm cursor-pointer transition-colors {selectedCategory?.id === sub.id ? 'text-blue-600 dark:text-blue-400 font-bold bg-blue-50 dark:bg-blue-900/20' : 'text-gray-500 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-800'}"
                  on:click={() => selectedCategory = sub}
                >
                  {sub.name}
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
          <div class="flex gap-2">
             <button class="p-2 bg-gray-100 dark:bg-gray-800 rounded-lg text-gray-500 hover:text-blue-600 transition-colors">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7" />
                </svg>
             </button>
             <button class="p-2 bg-gray-100 dark:bg-gray-800 rounded-lg text-gray-500 hover:text-blue-600 transition-colors">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
                </svg>
             </button>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8">
          {#each selectedCategory.books as book}
            <div class="group bg-white dark:bg-gray-900 p-8 rounded-3xl shadow-sm border border-gray-100 dark:border-gray-800 hover:border-blue-400 dark:hover:border-blue-500 transition-all hover:shadow-xl hover:-translate-y-1 flex flex-col h-full">
              <div class="mb-6 flex items-start justify-between">
                <div class="bg-blue-50 dark:bg-blue-900/30 p-3 rounded-2xl text-blue-600 dark:text-blue-400 group-hover:bg-blue-600 group-hover:text-white transition-colors">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                  </svg>
                </div>
                <div class="flex gap-1">
                  <span class="w-1 h-1 rounded-full bg-gray-200"></span>
                  <span class="w-1 h-1 rounded-full bg-gray-200"></span>
                  <span class="w-1 h-1 rounded-full bg-gray-200"></span>
                </div>
              </div>

              <h3 class="font-black text-xl mb-2 text-gray-900 dark:text-white leading-tight">{book.name}</h3>
              <p class="text-blue-600 dark:text-blue-400 text-sm font-bold mb-6 tracking-wide uppercase">{book.author}</p>

              <div class="mt-auto pt-6 border-t border-gray-50 dark:border-gray-800 flex items-center justify-between">
                <span class="text-xs font-bold text-gray-400 uppercase tracking-widest">PDF / EPUB</span>
                <button
                  class="bg-gray-900 dark:bg-white text-white dark:text-gray-900 px-6 py-2.5 rounded-xl text-sm font-black transition-all hover:bg-blue-600 hover:text-white group-hover:shadow-lg"
                  on:click={() => window.location.hash = `#/reader/${book.id}`}
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
