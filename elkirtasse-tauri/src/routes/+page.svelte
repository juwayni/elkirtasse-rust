<script lang="ts">
  import Library from "./Library.svelte";
  import Reader from "./Reader.svelte";
  import Search from "./Search.svelte";
  import { onMount } from "svelte";
  import { fade, fly } from 'svelte/transition';

  type View = "library" | "reader" | "search";
  let currentView: View = "library";
  let selectedBookId: string = "";
  let isDarkMode = false;

  function toggleDarkMode() {
    isDarkMode = !isDarkMode;
    if (isDarkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
    localStorage.setItem('theme', isDarkMode ? 'dark' : 'light');
  }

  function handleHashChange() {
    const hash = window.location.hash;
    if (hash.startsWith("#/reader/")) {
      selectedBookId = hash.replace("#/reader/", "").split("?")[0];
      currentView = "reader";
    } else if (hash === "#/search") {
      currentView = "search";
    } else {
      currentView = "library";
    }
  }

  onMount(() => {
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme === 'dark' || (!savedTheme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
      isDarkMode = true;
      document.documentElement.classList.add('dark');
    }
    window.addEventListener("hashchange", handleHashChange);
    handleHashChange();
    return () => window.removeEventListener("hashchange", handleHashChange);
  });
</script>

<div class="flex flex-col h-screen overflow-hidden bg-gray-50 dark:bg-gray-950 transition-colors duration-300">
  <nav class="bg-white dark:bg-gray-900 text-gray-900 dark:text-white px-6 py-4 border-b border-gray-200 dark:border-gray-800 shadow-sm flex items-center justify-between z-20">
    <div class="flex items-center gap-10">
      <div class="flex items-center gap-3">
        <div class="bg-blue-600 p-2 rounded-lg text-white">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
        </div>
        <h1 class="text-xl font-black tracking-tight uppercase">ELKIRTASSE <span class="text-blue-600">MODERN</span></h1>
      </div>

      <div class="flex gap-1 bg-gray-100 dark:bg-gray-800 p-1 rounded-xl">
        <button
          class="px-5 py-2 rounded-lg text-sm font-bold transition-all {currentView === 'library' ? 'bg-white dark:bg-gray-700 shadow-sm text-blue-600 dark:text-blue-400' : 'text-gray-500 hover:text-gray-900 dark:hover:text-white'}"
          on:click={() => window.location.hash = "#/"}
        >
          LIBRARY
        </button>
        <button
          class="px-5 py-2 rounded-lg text-sm font-bold transition-all {currentView === 'search' ? 'bg-white dark:bg-gray-700 shadow-sm text-blue-600 dark:text-blue-400' : 'text-gray-500 hover:text-gray-900 dark:hover:text-white'}"
          on:click={() => window.location.hash = "#/search"}
        >
          SEARCH
        </button>
      </div>
    </div>

    <div class="flex items-center gap-3">
      <button
        class="p-2.5 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-xl transition-colors text-gray-500 dark:text-gray-400"
        on:click={toggleDarkMode}
        title="Toggle Theme"
      >
        {#if isDarkMode}
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 9H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
          </svg>
        {:else}
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
          </svg>
        {/if}
      </button>
      <div class="h-6 w-px bg-gray-200 dark:bg-gray-800 mx-1"></div>
      <button class="p-2.5 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-xl transition-colors text-gray-500 dark:text-gray-400" title="Settings">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </button>
    </div>
  </nav>

  <div class="flex-1 overflow-hidden relative">
    {#key currentView}
      <div
        class="absolute inset-0"
        in:fly={{ y: 20, duration: 400, delay: 200 }}
        out:fade={{ duration: 200 }}
      >
        {#if currentView === "library"}
          <Library />
        {:else if currentView === "reader"}
          <Reader bookId={selectedBookId} />
        {:else if currentView === "search"}
          <Search />
        {/if}
      </div>
    {/key}
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  }

  :global(.dark) {
    color-scheme: dark;
  }
</style>
