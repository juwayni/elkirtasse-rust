<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { fade } from 'svelte/transition';

  export let bookId: string;

  interface Page {
    id: string;
    nass: string;
    part: string;
    page: string;
  }

  interface Chapter {
    id: string;
    title: string;
    level: number;
  }

  interface Annotation {
    id: string;
    page_id: string;
    text: string;
    color: string;
    note: string | null;
  }

  interface UserBookData {
    last_read_page: string;
    last_read_index: number;
    progress: number;
    bookmarks: string[];
  }

  let currentPageIndex = 0;
  let pages: Page[] = [];
  let chapters: Chapter[] = [];
  let annotations: Annotation[] = [];
  let bookmarks: string[] = [];
  let isLoading = true;
  let showToc = true;
  let selection: { text: string, range: Range } | null = null;

  // Reader Settings
  let fontSize = 20;
  let lineHeight = 1.8;
  let fontFamily = 'Amiri';
  let showSettings = false;

  // Search Context
  let highlightTerm = "";

  async function loadBook() {
    isLoading = true;
    try {
      // Get highlight term from URL if present
      const urlParams = new URLSearchParams(window.location.hash.split('?')[1]);
      highlightTerm = urlParams.get('q') || "";
      const targetPage = urlParams.get('page');

      const data: { pages: Page[], chapters: Chapter[] } = await invoke("get_book_content", { bookId });
      pages = data.pages;
      chapters = data.chapters;
      annotations = await invoke("get_annotations", { bookId });

      if (targetPage) {
        const index = pages.findIndex(p => p.page === targetPage);
        if (index !== -1) currentPageIndex = index;
      } else {
        const userData = await invoke<UserBookData | null>("get_user_book_data", { bookId });
        if (userData) {
            currentPageIndex = userData.last_read_index;
            bookmarks = userData.bookmarks || [];
        }
      }
    } catch (e) {
      console.error("Failed to load book", e);
    } finally {
      isLoading = false;
    }
  }

  function applyHighlight(html: string) {
    if (!highlightTerm) return html;
    const re = new RegExp(highlightTerm, 'gi');
    return html.replace(re, '<mark class="bg-yellow-200 dark:bg-yellow-900/50 rounded px-1">$0</mark>');
  }

  function saveSettings() {
    localStorage.setItem('readerSettings', JSON.stringify({ fontSize, lineHeight, fontFamily }));
  }

  function loadSettings() {
    const saved = localStorage.getItem('readerSettings');
    if (saved) {
      const parsed = JSON.parse(saved);
      fontSize = parsed.fontSize || 20;
      lineHeight = parsed.lineHeight || 1.8;
      fontFamily = parsed.fontFamily || 'Amiri';
    }
  }

  function handleSelection() {
    const s = window.getSelection();
    if (s && s.rangeCount > 0 && !s.isCollapsed) {
      const range = s.getRangeAt(0);
      selection = { text: s.toString(), range };
    } else {
      selection = null;
    }
  }

  async function addHighlight(color: string) {
    if (!selection) return;
    const pageId = pages[currentPageIndex].id;
    try {
      const newAnnotation: Annotation = await invoke("add_annotation", {
        bookId,
        pageId,
        text: selection.text,
        color,
        note: null
      });
      annotations = [...annotations, newAnnotation];
      selection = null;
      window.getSelection()?.removeAllRanges();
    } catch (e) {
      console.error("Failed to add highlight", e);
    }
  }

  async function copySelection() {
    if (selection) {
       await navigator.clipboard.writeText(selection.text);
       selection = null;
       window.getSelection()?.removeAllRanges();
    }
  }

  async function toggleBookmark() {
    const pageId = pages[currentPageIndex].id;
    try {
        const isBookmarked = await invoke("toggle_bookmark", { bookId, pageId });
        if (isBookmarked) {
            bookmarks = [...bookmarks, pageId];
        } else {
            bookmarks = bookmarks.filter(id => id !== pageId);
        }
    } catch (e) {
        console.error("Failed to toggle bookmark", e);
    }
  }

  async function updateProgress() {
    if (pages.length === 0) return;
    const progress = currentPageIndex / (pages.length - 1);
    try {
        await invoke("save_user_book_data", {
            bookId,
            lastReadPage: pages[currentPageIndex].page,
            lastReadIndex: currentPageIndex,
            progress
        });
    } catch (e) {
        console.error("Failed to save progress", e);
    }
  }

  $: if (currentPageIndex !== undefined) {
    updateProgress();
  }

  onMount(() => {
    loadBook();
    loadSettings();
  });
</script>

<div role="none" class="flex h-full bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100" on:mouseup={handleSelection}>
  {#if showToc}
    <aside class="w-64 border-r border-gray-200 dark:border-gray-800 overflow-y-auto p-4 bg-gray-50 dark:bg-gray-900">
      <h2 class="text-lg font-bold mb-4">Contents</h2>
      <ul>
        {#each chapters as chapter}
          <li class="mb-1" style="margin-left: {(chapter.level - 1) * 12}px">
            <button
              class="text-left w-full p-1 text-sm hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
              on:click={() => {
                 const index = pages.findIndex(p => p.id === chapter.id);
                 if (index !== -1) currentPageIndex = index;
              }}
            >
              {chapter.title}
            </button>
          </li>
        {/each}
      </ul>
    </aside>
  {/if}

  <main class="flex-1 flex flex-col relative">
    <header class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 sticky top-0 z-10">
      <div class="flex items-center gap-4">
        <button
          class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded transition-colors"
          on:click={() => showToc = !showToc}
          aria-label="Toggle Table of Contents"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
          </svg>
        </button>
        <span class="text-sm font-medium">Page {pages[currentPageIndex]?.page || '-'} | Part {pages[currentPageIndex]?.part || '-'}</span>
        <button
          class="p-2 rounded-full transition-colors {bookmarks.includes(pages[currentPageIndex]?.id) ? 'text-red-500 hover:bg-red-50' : 'text-gray-400 hover:bg-gray-100'}"
          on:click={toggleBookmark}
          title="Toggle Bookmark"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill={bookmarks.includes(pages[currentPageIndex]?.id) ? "currentColor" : "none"} viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
          </svg>
        </button>
      </div>

      <div class="flex items-center gap-4">
        {#if selection}
          <div class="flex gap-1 bg-gray-100 dark:bg-gray-800 p-1 rounded-lg mr-2" transition:fade>
            <button on:click={() => addHighlight('yellow')} class="w-7 h-7 rounded-lg bg-yellow-300 border border-gray-300 shadow-sm" aria-label="Highlight Yellow"></button>
            <button on:click={() => addHighlight('green')} class="w-7 h-7 rounded-lg bg-green-300 border border-gray-300 shadow-sm" aria-label="Highlight Green"></button>
            <button on:click={copySelection} class="px-3 py-1 text-xs font-bold uppercase tracking-wider hover:bg-gray-200 dark:hover:bg-gray-700 rounded transition-colors">Copy</button>
          </div>
        {/if}

        <button
          class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded transition-colors"
          on:click={() => window.print()}
          title="Print this page"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" />
          </svg>
        </button>

        <button
          class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded transition-colors"
          on:click={() => showSettings = !showSettings}
          aria-label="Reader Settings"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
          </svg>
        </button>

        <div class="flex items-center gap-2 border-l pl-4 dark:border-gray-800">
            <button
              class="px-4 py-2 border border-gray-300 dark:border-gray-700 rounded hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors disabled:opacity-50"
              on:click={() => { if (currentPageIndex > 0) currentPageIndex--; }}
              disabled={currentPageIndex === 0}
            >
              Prev
            </button>
            <button
              class="px-4 py-2 border border-gray-300 dark:border-gray-700 rounded hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors disabled:opacity-50"
              on:click={() => { if (currentPageIndex < pages.length - 1) currentPageIndex++; }}
              disabled={currentPageIndex === pages.length - 1}
            >
              Next
            </button>
        </div>
      </div>
    </header>

    {#if showSettings}
      <div class="absolute top-16 right-4 w-72 bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-2xl shadow-2xl p-6 z-20" transition:fade>
        <h3 class="font-bold mb-4 text-xs uppercase tracking-widest text-gray-400">Typography Settings</h3>
        <div class="space-y-6">
          <div>
            <label for="font-family" class="text-sm font-medium mb-2 block">Font Family</label>
            <select id="font-family" bind:value={fontFamily} on:change={saveSettings} class="w-full p-2 rounded-lg bg-gray-100 dark:bg-gray-800 border-none outline-none">
              <option value="Amiri">Amiri (Traditional)</option>
              <option value="Scheherazade New">Scheherazade (Naskh)</option>
              <option value="system-ui">System Default</option>
            </select>
          </div>
          <div>
            <label for="font-size" class="text-sm font-medium mb-2 block">Font Size ({fontSize}px)</label>
            <input id="font-size" type="range" min="12" max="48" bind:value={fontSize} on:input={saveSettings} class="w-full">
          </div>
          <div>
            <label for="line-height" class="text-sm font-medium mb-2 block">Line Height ({lineHeight})</label>
            <input id="line-height" type="range" min="1" max="3" step="0.1" bind:value={lineHeight} on:input={saveSettings} class="w-full">
          </div>
        </div>
      </div>
    {/if}

    <div class="flex-1 overflow-y-auto p-8 md:p-12 lg:p-16 max-w-4xl mx-auto w-full">
      {#if isLoading}
        <div class="flex items-center justify-center h-full">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
        </div>
      {:else if pages.length > 0}
        <article
          class="prose prose-lg dark:prose-invert max-w-none text-right leading-loose transition-all duration-300"
          dir="rtl"
          style="font-family: '{fontFamily}', serif; font-size: {fontSize}px; line-height: {lineHeight}"
        >
          {@html applyHighlight(pages[currentPageIndex].nass)}
        </article>

        {#each annotations.filter(a => a.page_id === pages[currentPageIndex].id) as anno}
          <div class="mt-8 mb-4 p-4 rounded-xl bg-gray-50 dark:bg-gray-900 border-l-4 border-blue-500 shadow-sm transition-all hover:shadow-md">
            <p class="text-sm font-medium text-gray-500 mb-2 uppercase tracking-tighter">Highlight</p>
            <p class="text-lg italic text-right font-serif" dir="rtl">"{anno.text}"</p>
          </div>
        {/each}
      {:else}
        <p class="text-center text-gray-500 mt-20">Book content not found.</p>
      {/if}
    </div>
  </main>
</div>

<style>
  :global(.prose) {
    max-width: none !important;
  }
</style>
