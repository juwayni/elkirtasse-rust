<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { fade, slide } from 'svelte/transition';

  export let onOpenBook: (bookId: string, title: string, page?: string) => void;

  interface QuranNode {
    id: string;
    name: string;
    children: QuranNode[];
    leaves: { id: string, name: string }[];
  }

  interface Surah {
    id: string;
    name: string;
    ayas: { id: string, text: string }[];
  }

  let nodes: QuranNode[] = [];
  let surahs: Surah[] = [];
  let viewMode: 'ajzaa' | 'surahs' = 'ajzaa';
  let isLoading = true;
  let expandedNodes = new Set<string>();
  let selectedSurah: Surah | null = null;
  let searchQuery = "";

  async function loadQuran() {
    isLoading = true;
    try {
      nodes = await invoke<QuranNode[]>("get_ajzaa");
      surahs = await invoke<Surah[]>("get_curan");
    } catch (e) {
      console.error("Failed to load Quran data", e);
    } finally {
      isLoading = false;
    }
  }

  function toggleNode(id: string) {
    if (expandedNodes.has(id)) {
      expandedNodes.delete(id);
    } else {
      expandedNodes.add(id);
    }
    expandedNodes = expandedNodes;
  }

  onMount(loadQuran);

  $: filteredSurahs = surahs.filter((s: Surah) => s.name.includes(searchQuery));
</script>

<div class="flex flex-col h-full bg-white dark:bg-gray-950">
  <header class="p-6 border-b border-gray-100 dark:border-gray-800">
    <div class="flex items-center justify-between mb-6">
      <h1 class="text-3xl font-extrabold tracking-tight text-gray-900 dark:text-white">The Holy Quran</h1>
      <div class="flex bg-gray-100 dark:bg-gray-800 p-1 rounded-xl">
        <button
          class="px-4 py-2 rounded-lg text-sm font-bold transition-all {viewMode === 'ajzaa' ? 'bg-white dark:bg-gray-700 shadow-sm text-blue-600' : 'text-gray-500 hover:text-gray-700'}"
          on:click={() => viewMode = 'ajzaa'}
        >
          Parts (Ajza'a)
        </button>
        <button
          class="px-4 py-2 rounded-lg text-sm font-bold transition-all {viewMode === 'surahs' ? 'bg-white dark:bg-gray-700 shadow-sm text-blue-600' : 'text-gray-500 hover:text-gray-700'}"
          on:click={() => viewMode = 'surahs'}
        >
          Chapters (Surahs)
        </button>
      </div>
    </div>

    <div class="relative">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search Surahs..."
        class="w-full pl-12 pr-4 py-3 rounded-2xl bg-gray-50 dark:bg-gray-900 border-none focus:ring-2 focus:ring-blue-500 transition-all outline-none"
      />
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 absolute left-4 top-3.5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
    </div>
  </header>

  <div class="flex-1 overflow-y-auto p-6">
    {#if isLoading}
      <div class="flex items-center justify-center h-full">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>
    {:else if viewMode === 'ajzaa'}
      <div class="max-w-3xl mx-auto space-y-2">
        {#each nodes as node}
          <div class="border border-gray-100 dark:border-gray-800 rounded-2xl overflow-hidden">
            <button
              class="w-full flex items-center justify-between p-4 hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors"
              on:click={() => toggleNode(node.id)}
            >
              <span class="font-bold text-lg">{node.name}</span>
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 transform transition-transform {expandedNodes.has(node.id) ? 'rotate-180' : ''}" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </button>
            {#if expandedNodes.has(node.id)}
              <div class="p-4 bg-gray-50 dark:bg-gray-900/50 space-y-4" transition:slide>
                {#each node.children as child}
                  <div>
                    <h4 class="text-sm font-black text-blue-500 uppercase tracking-widest mb-2">{child.name}</h4>
                    <div class="grid grid-cols-2 md:grid-cols-3 gap-2">
                      {#each child.leaves as leaf}
                        <button
                          class="p-3 text-right text-sm bg-white dark:bg-gray-800 rounded-xl border border-gray-100 dark:border-gray-700 hover:border-blue-400 hover:shadow-md transition-all font-serif"
                          dir="rtl"
                          on:click={() => onOpenBook("curan", "القرآن الكريم", leaf.id)}
                        >
                          {leaf.name}
                        </button>
                      {/each}
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each filteredSurahs as surah}
          <button
            class="group p-6 text-right bg-white dark:bg-gray-900 rounded-3xl border border-gray-100 dark:border-gray-800 hover:border-blue-500 hover:shadow-2xl transition-all duration-300"
            on:click={() => selectedSurah = surah}
          >
            <div class="flex items-center justify-between mb-4">
              <span class="w-10 h-10 flex items-center justify-center rounded-xl bg-blue-50 dark:bg-blue-900/30 text-blue-600 font-black text-xs">{surah.id}</span>
              <h3 class="text-2xl font-serif font-bold group-hover:text-blue-500 transition-colors" dir="rtl">{surah.name}</h3>
            </div>
            <p class="text-sm text-gray-500">{surah.ayas.length} Verses (Ayahs)</p>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  {#if selectedSurah}
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm" transition:fade>
      <div class="bg-white dark:bg-gray-900 w-full max-w-4xl max-h-[90vh] rounded-3xl shadow-2xl flex flex-col overflow-hidden" transition:slide>
        <header class="p-6 border-b border-gray-100 dark:border-gray-800 flex items-center justify-between bg-white dark:bg-gray-950">
          <h2 class="text-3xl font-serif font-bold" dir="rtl">{selectedSurah?.name || ''}</h2>
          <button on:click={() => selectedSurah = null} class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-full transition-colors" aria-label="Close">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </header>
        <div class="flex-1 overflow-y-auto p-8 space-y-8" dir="rtl">
          {#each selectedSurah?.ayas || [] as aya}
            <div class="group relative">
              <span class="absolute -right-8 top-1 text-xs font-black text-gray-300 dark:text-gray-700">{aya.id}</span>
              <p class="text-3xl font-serif leading-loose text-right text-gray-800 dark:text-gray-200 hover:text-blue-600 transition-colors cursor-pointer">
                {aya.text}
              </p>
            </div>
          {/each}
        </div>
        <footer class="p-6 border-t border-gray-100 dark:border-gray-800 flex justify-end">
          {#if selectedSurah}
            <button
              class="px-8 py-3 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded-2xl shadow-lg shadow-blue-500/30 transition-all"
              on:click={() => selectedSurah && onOpenBook("curan", "القرآن الكريم", selectedSurah.id)}
            >
              Read in Library
            </button>
          {/if}
        </footer>
      </div>
    </div>
  {/if}
</div>
