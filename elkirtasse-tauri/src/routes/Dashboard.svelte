<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { fade, fly } from 'svelte/transition';

  interface Book {
    id: string;
    name: string;
    author: string;
  }

  let recentBooks: Book[] = [];
  let isIndexing = false;

  async function loadDashboard() {
    // Recent books could be fetched from localStorage or a local DB
    const saved = localStorage.getItem('recentBooks');
    if (saved) recentBooks = JSON.parse(saved);
  }

  async function startGlobalIndex() {
    isIndexing = true;
    try {
        await invoke("start_indexing");
        alert("Library indexing complete!");
    } catch (e) {
        console.error("Indexing failed", e);
    } finally {
        isIndexing = false;
    }
  }

  onMount(loadDashboard);
</script>

<div class="h-full bg-white dark:bg-gray-950 p-8 lg:p-16 overflow-y-auto transition-colors duration-300">
  <div class="max-w-5xl mx-auto">
    <header class="mb-16" in:fly={{ y: -20, duration: 800 }}>
      <h2 class="text-xs font-black tracking-[0.4em] text-blue-600 uppercase mb-4">Welcome Back</h2>
      <h1 class="text-6xl font-black text-gray-900 dark:text-white tracking-tighter">Your Digital <span class="text-blue-600">Sanctuary</span></h1>
      <p class="mt-6 text-xl text-gray-500 max-w-2xl font-medium leading-relaxed">Continue your journey through the world's greatest Islamic literature with Elkirtasse Modern.</p>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-12 mb-20">
      <section in:fly={{ x: -20, duration: 800, delay: 200 }}>
        <h3 class="text-sm font-black uppercase tracking-widest text-gray-400 mb-8 flex items-center gap-3">
            <span class="w-8 h-px bg-gray-200"></span> Recent Activity
        </h3>
        {#if recentBooks.length > 0}
            <div class="space-y-4">
                {#each recentBooks as book}
                    <div class="group bg-gray-50 dark:bg-gray-900 p-6 rounded-3xl border border-transparent hover:border-blue-400/30 transition-all cursor-pointer shadow-sm hover:shadow-xl" on:click={() => window.location.hash = `#/reader/${book.id}`}>
                        <h4 class="font-black text-lg group-hover:text-blue-600 transition-colors">{book.name}</h4>
                        <p class="text-sm text-gray-500 font-bold uppercase tracking-wider mt-1">{book.author}</p>
                    </div>
                {/each}
            </div>
        {:else}
            <div class="bg-gray-50 dark:bg-gray-900 p-12 rounded-3xl border-2 border-dashed border-gray-200 dark:border-gray-800 text-center">
                <p class="text-gray-400 font-bold">No recent books yet</p>
                <button class="mt-4 text-blue-600 font-black hover:underline" on:click={() => window.location.hash = "#/"}>EXPLORE LIBRARY</button>
            </div>
        {/if}
      </section>

      <section in:fly={{ x: 20, duration: 800, delay: 400 }}>
        <h3 class="text-sm font-black uppercase tracking-widest text-gray-400 mb-8 flex items-center gap-3">
            <span class="w-8 h-px bg-gray-200"></span> Quick Actions
        </h3>
        <div class="grid grid-cols-1 gap-4">
            <button
                class="flex items-center justify-between p-6 bg-blue-600 text-white rounded-3xl font-black text-lg shadow-lg hover:shadow-blue-500/40 hover:-translate-y-1 transition-all active:scale-95 disabled:opacity-50"
                on:click={startGlobalIndex}
                disabled={isIndexing}
            >
                <span>{isIndexing ? 'INDEXING...' : 'REBUILD SEARCH INDEX'}</span>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 {isIndexing ? 'animate-spin' : ''}" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
            </button>
            <button class="flex items-center justify-between p-6 bg-gray-900 dark:bg-white text-white dark:text-gray-900 rounded-3xl font-black text-lg shadow-lg hover:-translate-y-1 transition-all active:scale-95">
                <span>EXPORT ANNOTATIONS</span>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                </svg>
            </button>
        </div>
      </section>
    </div>

    <footer class="border-t border-gray-100 dark:border-gray-900 pt-12 flex justify-between items-center opacity-40 hover:opacity-100 transition-opacity">
        <p class="text-xs font-bold uppercase tracking-widest text-gray-500">ELKIRTASSE MODERN ENGINE v1.0</p>
        <div class="flex gap-6">
            <a href="#" class="text-xs font-bold uppercase tracking-widest hover:text-blue-600 transition-colors">Documentation</a>
            <a href="#" class="text-xs font-bold uppercase tracking-widest hover:text-blue-600 transition-colors">Privacy</a>
        </div>
    </footer>
  </div>
</div>
