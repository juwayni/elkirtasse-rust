<script lang="ts">
  import Library from "./Library.svelte";
  import Reader from "./Reader.svelte";
  import Search from "./Search.svelte";
  import Dashboard from "./Dashboard.svelte";
  import Quran from "./Quran.svelte";
  import Narrators from "./Narrators.svelte";
  import Tabs from "./Tabs.svelte";
  import { onMount } from "svelte";
  import { fade, fly } from 'svelte/transition';

  type View = "library" | "reader" | "search" | "dashboard" | "quran" | "narrators";
  let currentView: View = "dashboard";
  let isDarkMode = false;

  interface Tab {
    id: string;
    title: string;
    type: View;
  }

  let tabs: Tab[] = [
    { id: 'dashboard', title: 'DASHBOARD', type: 'dashboard' },
    { id: 'library', title: 'LIBRARY', type: 'library' },
    { id: 'search', title: 'SEARCH', type: 'search' },
    { id: 'quran', title: 'QURAN', type: 'quran' },
    { id: 'narrators', title: 'NARRATORS', type: 'narrators' }
  ];
  let activeTabId = 'dashboard';

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
      const parts = hash.replace("#/reader/", "").split("?");
      const bookId = parts[0];
      const existingTab = tabs.find(t => t.id === bookId);
      if (!existingTab) {
        tabs = [...tabs, { id: bookId, title: 'Reader', type: 'reader' }];
      }
      activeTabId = bookId;
      currentView = 'reader';
    } else if (hash === "#/search") {
      activeTabId = 'search';
      currentView = "search";
    } else if (hash === "#/quran") {
      activeTabId = 'quran';
      currentView = "quran";
    } else if (hash === "#/narrators") {
      activeTabId = 'narrators';
      currentView = "narrators";
    } else if (hash === "#/library" || hash === "#/") {
        activeTabId = 'library';
        currentView = "library";
    } else {
      activeTabId = 'dashboard';
      currentView = "dashboard";
    }
  }

  function switchTab(id: string) {
    const tab = tabs.find(t => t.id === id);
    if (tab) {
        activeTabId = id;
        currentView = tab.type;
        if (tab.type === 'reader') {
            window.location.hash = `#/reader/${tab.id}`;
        } else if (tab.type === 'search') {
            window.location.hash = "#/search";
        } else if (tab.type === 'quran') {
            window.location.hash = "#/quran";
        } else if (tab.type === 'narrators') {
            window.location.hash = "#/narrators";
        } else if (tab.type === 'library') {
            window.location.hash = "#/library";
        } else {
            window.location.hash = "#/dashboard";
        }
    }
  }

  function closeTab(id: string) {
    if (id === 'library' || id === 'search' || id === 'dashboard' || id === 'quran' || id === 'narrators') return;
    const index = tabs.findIndex(t => t.id === id);
    tabs = tabs.filter(t => t.id !== id);
    if (activeTabId === id) {
        switchTab(tabs[index - 1]?.id || 'dashboard');
    }
  }

  function openBook(bookId: string, title: string, page?: string) {
     const existingTab = tabs.find(t => t.id === bookId);
     if (!existingTab) {
        tabs = [...tabs, { id: bookId, title: title, type: 'reader' }];
     }
     window.location.hash = `#/reader/${bookId}${page ? '?page=' + page : ''}`;
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

<div class="flex flex-col h-screen overflow-hidden bg-gray-50 dark:bg-gray-950 transition-colors duration-500 font-sans">
  <nav class="bg-white/80 dark:bg-gray-900/80 backdrop-blur-xl text-gray-900 dark:text-white px-8 py-5 border-b border-gray-100 dark:border-gray-800 shadow-sm flex items-center justify-between z-20">
    <div class="flex items-center gap-12">
      <div
        role="button"
        tabindex="0"
        class="flex items-center gap-4 cursor-pointer group"
        on:click={() => window.location.hash = "#/dashboard"}
        on:keydown={(e) => e.key === 'Enter' && (window.location.hash = "#/dashboard")}
      >
        <div class="bg-blue-600 p-2.5 rounded-2xl text-white shadow-lg shadow-blue-500/30 group-hover:scale-110 transition-transform">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
        </div>
        <h1 class="text-2xl font-black tracking-tighter uppercase italic">ELKIRTASSE <span class="text-blue-600 not-italic">MODERN</span></h1>
      </div>
    </div>

    <div class="flex items-center gap-5">
      <button
        class="p-3 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-2xl transition-all text-gray-500 dark:text-gray-400 active:scale-90"
        on:click={toggleDarkMode}
        aria-label="Toggle Dark Mode"
      >
        {#if isDarkMode}
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 9H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
          </svg>
        {:else}
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
          </svg>
        {/if}
      </button>
    </div>
  </nav>

  <Tabs {tabs} activeTabId={activeTabId} onSwitch={switchTab} onClose={closeTab} />

  <div class="flex-1 overflow-hidden relative">
    {#each tabs as tab}
      <div
        class="absolute inset-0 transition-all duration-500 {activeTabId === tab.id ? 'z-10 opacity-100' : 'z-0 pointer-events-none opacity-0 translate-y-4'}"
      >
        {#if tab.type === "library"}
          <Library onOpenBook={openBook} />
        {:else if tab.type === "reader"}
          <Reader bookId={tab.id} />
        {:else if tab.type === "search"}
          <Search onOpenBook={openBook} />
        {:else if tab.type === "dashboard"}
          <Dashboard onOpenBook={openBook} />
        {:else if tab.type === "quran"}
          <Quran onOpenBook={openBook} />
        {:else if tab.type === "narrators"}
          <Narrators />
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  /* Inter Font */
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;700;900&family=Amiri:ital,wght@0,400;0,700;1,400;1,700&display=swap');
</style>
