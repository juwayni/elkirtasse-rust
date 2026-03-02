<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { fade, slide } from 'svelte/transition';

  interface Narrator {
    id: string;
    name: string;
    bio: string;
  }

  let narrators: Narrator[] = [];
  let isLoading = true;
  let searchQuery = "";
  let selectedNarrator: Narrator | null = null;

  async function loadNarrators() {
    isLoading = true;
    try {
      narrators = await invoke("get_narrators");
    } catch (e) {
      console.error("Failed to load narrators", e);
    } finally {
      isLoading = false;
    }
  }

  onMount(loadNarrators);

  $: filteredNarrators = narrators.filter(n => n.name.includes(searchQuery));
</script>

<div class="flex flex-col h-full bg-white dark:bg-gray-950">
  <header class="p-6 border-b border-gray-100 dark:border-gray-800">
    <h1 class="text-3xl font-extrabold tracking-tight text-gray-900 dark:text-white mb-6">Narrators (Rowat)</h1>
    <div class="relative">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search for a narrator..."
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
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {#each filteredNarrators as narrator}
          <button
            class="p-6 text-right bg-white dark:bg-gray-900 rounded-3xl border border-gray-100 dark:border-gray-800 hover:border-blue-500 hover:shadow-2xl transition-all duration-300"
            on:click={() => selectedNarrator = narrator}
          >
            <h3 class="text-xl font-serif font-bold mb-2" dir="rtl">{narrator.name}</h3>
            <p class="text-xs text-gray-500 uppercase tracking-widest">ID: {narrator.id}</p>
          </button>
        {:else}
          <div class="col-span-full text-center py-20">
            <p class="text-gray-500">No narrators found matching your search.</p>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  {#if selectedNarrator}
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm" transition:fade>
      <div class="bg-white dark:bg-gray-900 w-full max-w-4xl max-h-[90vh] rounded-3xl shadow-2xl flex flex-col overflow-hidden" transition:slide>
        <header class="p-6 border-b border-gray-100 dark:border-gray-800 flex items-center justify-between bg-white dark:bg-gray-950">
          <h2 class="text-3xl font-serif font-bold" dir="rtl">{selectedNarrator.name}</h2>
          <button on:click={() => selectedNarrator = null} class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-full transition-colors" aria-label="Close">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </header>
        <div class="flex-1 overflow-y-auto p-8" dir="rtl">
          <div class="prose prose-lg dark:prose-invert max-w-none font-serif leading-relaxed">
            {@html selectedNarrator.bio}
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>
