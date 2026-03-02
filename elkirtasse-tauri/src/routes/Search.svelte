<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  export let onOpenBook: (bookId: string, title: string, page?: string) => void;

  interface SearchResult {
    book_id: string;
    book_title: string;
    author: string;
    part: string;
    page: string;
    snippet: string;
  }

  let query = "";
  let results: SearchResult[] = [];
  let isSearching = false;
  let bookFilter: string[] = [];

  async function performSearch() {
    if (!query.trim()) return;
    isSearching = true;
    try {
      results = await invoke("search_books", {
        query,
        bookFilter: bookFilter.length > 0 ? bookFilter : null
      });
    } catch (e) {
      console.error("Search failed", e);
    } finally {
      isSearching = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter") {
      performSearch();
    }
  }
</script>

<div class="h-full flex flex-col bg-gray-50 dark:bg-gray-950">
  <div class="p-6 border-b border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 shadow-sm z-10">
    <div class="max-w-4xl mx-auto flex flex-col gap-4">
      <div class="flex gap-4">
        <input
          type="text"
          bind:value={query}
          on:keydown={handleKeydown}
          placeholder="Search for books, authors, or text..."
          class="flex-1 p-4 rounded-xl border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 focus:ring-2 focus:ring-blue-500 outline-none transition-all shadow-inner"
        />
        <button
          on:click={performSearch}
          disabled={isSearching}
          class="bg-blue-600 hover:bg-blue-700 text-white px-8 py-4 rounded-xl font-bold transition-all disabled:opacity-50 flex items-center gap-3 shadow-lg hover:shadow-blue-500/20 active:scale-95"
        >
          {#if isSearching}
            <div class="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
          {/if}
          SEARCH
        </button>
      </div>

      <div class="flex gap-4 text-xs font-medium text-gray-500 uppercase tracking-widest">
        <span>Options:</span>
        <label class="flex items-center gap-2 cursor-pointer hover:text-blue-600 transition-colors">
          <input type="checkbox" class="rounded border-gray-300"> Full Match
        </label>
        <label class="flex items-center gap-2 cursor-pointer hover:text-blue-600 transition-colors">
          <input type="checkbox" class="rounded border-gray-300"> Diacritics Sensitive
        </label>
      </div>
    </div>
  </div>

  <div class="flex-1 overflow-y-auto p-6 lg:p-12">
    <div class="max-w-4xl mx-auto space-y-8">
      {#if results.length > 0}
        <div class="flex items-center justify-between">
          <p class="text-sm font-medium text-gray-400">Showing {results.length} relevant results</p>
          <div class="flex gap-2">
            <button class="px-3 py-1 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg text-xs hover:bg-gray-50 transition-colors">Sort by Relevance</button>
            <button class="px-3 py-1 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg text-xs hover:bg-gray-50 transition-colors">Sort by Date</button>
          </div>
        </div>

        {#each results as result}
          <div class="bg-white dark:bg-gray-800 p-8 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700 hover:border-blue-300 dark:hover:border-blue-600 transition-all group relative overflow-hidden">
            <div class="absolute top-0 left-0 w-1 h-full bg-blue-500 opacity-0 group-hover:opacity-100 transition-opacity"></div>

            <div class="flex justify-between items-start mb-6">
              <div class="flex-1">
                <button
                  class="text-left w-full mb-2"
                  on:click={() => onOpenBook(result.book_id, result.book_title, result.page)}
                >
                   <h3 class="text-2xl font-black text-gray-900 dark:text-white group-hover:text-blue-600 transition-colors cursor-pointer">
                    {result.book_title}
                   </h3>
                </button>
                <p class="text-base text-blue-600 dark:text-blue-400 font-medium">{result.author}</p>
              </div>
              <div class="flex flex-col items-end gap-1">
                <span class="text-xs font-bold text-gray-400 bg-gray-50 dark:bg-gray-900 px-3 py-1.5 rounded-full border border-gray-100 dark:border-gray-700">PART {result.part}</span>
                <span class="text-xs font-bold text-gray-400 bg-gray-50 dark:bg-gray-900 px-3 py-1.5 rounded-full border border-gray-100 dark:border-gray-700">PAGE {result.page}</span>
              </div>
            </div>

            <div class="relative">
              <div class="absolute -left-4 top-0 bottom-0 w-0.5 bg-gray-100 dark:bg-gray-700"></div>
              <p class="text-gray-700 dark:text-gray-300 leading-relaxed text-xl text-right font-serif" dir="rtl">
                ...{@html result.snippet}...
              </p>
            </div>

            <div class="mt-8 flex justify-end gap-4 border-t border-gray-50 dark:border-gray-700 pt-6">
              <button class="text-gray-400 hover:text-blue-600 transition-colors" title="Save result">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
                </svg>
              </button>
              <button
                class="bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-white px-6 py-2.5 rounded-xl font-bold hover:bg-blue-600 hover:text-white transition-all text-sm border border-gray-100 dark:border-gray-700 shadow-sm"
                on:click={() => onOpenBook(result.book_id, result.book_title, result.page)}
              >
                OPEN BOOK →
              </button>
            </div>
          </div>
        {/each}
      {:else if !isSearching && query}
        <div class="flex flex-col items-center justify-center py-32 text-gray-400">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-20 w-20 mb-6 opacity-20" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 9.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <p class="text-2xl font-medium mb-2">No matches found</p>
          <p class="text-sm opacity-60">Try adjusting your keywords or filters</p>
        </div>
      {:else if !isSearching}
         <div class="flex flex-col items-center justify-center py-32 text-gray-300">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-32 w-32 mb-8 opacity-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <p class="text-xl font-light">Search the vast Elkirtasse library instantly</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  :global(.prose em) {
    background-color: rgba(59, 130, 246, 0.2);
    padding: 0 4px;
    border-radius: 4px;
    font-style: normal;
    font-weight: bold;
    color: #2563eb;
  }
</style>
