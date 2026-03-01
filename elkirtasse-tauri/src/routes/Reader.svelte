<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

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

  let currentPageIndex = 0;
  let pages: Page[] = [];
  let chapters: Chapter[] = [];
  let annotations: Annotation[] = [];
  let isLoading = true;
  let showToc = true;
  let selection: { text: string, range: Range } | null = null;

  async function loadBook() {
    isLoading = true;
    try {
      const data: { pages: Page[], chapters: Chapter[] } = await invoke("get_book_content", { bookId });
      pages = data.pages;
      chapters = data.chapters;
      annotations = await invoke("get_annotations", { bookId });
    } catch (e) {
      console.error("Failed to load book", e);
    } finally {
      isLoading = false;
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

  function nextPage() {
    if (currentPageIndex < pages.length - 1) currentPageIndex++;
  }

  function prevPage() {
    if (currentPageIndex > 0) currentPageIndex--;
  }

  function goToChapter(chapterId: string) {
    const index = pages.findIndex(p => p.id === chapterId);
    if (index !== -1) currentPageIndex = index;
  }

  onMount(loadBook);
</script>

<div class="flex h-full bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100" on:mouseup={handleSelection}>
  {#if showToc}
    <aside class="w-64 border-r border-gray-200 dark:border-gray-700 overflow-y-auto p-4 bg-gray-50 dark:bg-gray-900">
      <h2 class="text-lg font-bold mb-4">Contents</h2>
      <ul>
        {#each chapters as chapter}
          <li class="mb-1" style="margin-left: {(chapter.level - 1) * 12}px">
            <button
              class="text-left w-full p-1 text-sm hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
              on:click={() => goToChapter(chapter.id)}
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
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
          </svg>
        </button>
        <span class="text-sm font-medium">Page {pages[currentPageIndex]?.page || '-'} | Part {pages[currentPageIndex]?.part || '-'}</span>
      </div>

      <div class="flex items-center gap-2">
        {#if selection}
          <div class="flex gap-1 mr-4 bg-gray-100 dark:bg-gray-800 p-1 rounded-lg">
            <button on:click={() => addHighlight('yellow')} class="w-6 h-6 rounded-full bg-yellow-300 border border-gray-300"></button>
            <button on:click={() => addHighlight('green')} class="w-6 h-6 rounded-full bg-green-300 border border-gray-300"></button>
            <button on:click={() => addHighlight('blue')} class="w-6 h-6 rounded-full bg-blue-300 border border-gray-300"></button>
          </div>
        {/if}
        <button
          class="px-4 py-2 border border-gray-300 dark:border-gray-700 rounded hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors disabled:opacity-50"
          on:click={prevPage}
          disabled={currentPageIndex === 0}
        >
          Previous
        </button>
        <button
          class="px-4 py-2 border border-gray-300 dark:border-gray-700 rounded hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors disabled:opacity-50"
          on:click={nextPage}
          disabled={currentPageIndex === pages.length - 1}
        >
          Next
        </button>
      </div>
    </header>

    <div class="flex-1 overflow-y-auto p-8 md:p-12 lg:p-16 max-w-4xl mx-auto w-full">
      {#if isLoading}
        <div class="flex items-center justify-center h-full">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
        </div>
      {:else if pages.length > 0}
        <article class="prose prose-lg dark:prose-invert max-w-none font-serif text-right leading-loose" dir="rtl">
          {@html pages[currentPageIndex].nass}
        </article>

        {#if annotations.filter(a => a.page_id === pages[currentPageIndex].id).length > 0}
          <div class="mt-12 border-t border-gray-200 pt-6">
            <h3 class="text-sm font-bold uppercase tracking-wider text-gray-500 mb-4">Highlights & Notes</h3>
            {#each annotations.filter(a => a.page_id === pages[currentPageIndex].id) as anno}
              <div class="mb-4 p-3 rounded bg-{anno.color}-50 border-l-4 border-{anno.color}-400 text-sm italic">
                "{anno.text}"
              </div>
            {/each}
          </div>
        {/if}
      {:else}
        <p class="text-center text-gray-500 mt-20">Book content not found.</p>
      {/if}
    </div>
  </main>
</div>

<style>
  article {
    font-family: 'Amiri', 'Traditional Arabic', serif;
    font-size: 1.35rem;
  }
  .bg-yellow-50 { background-color: #fefce8; }
  .border-yellow-400 { border-color: #facc15; }
  .bg-green-50 { background-color: #f0fdf4; }
  .border-green-400 { border-color: #4ade80; }
  .bg-blue-50 { background-color: #eff6ff; }
  .border-blue-400 { border-color: #60a5fa; }
</style>
