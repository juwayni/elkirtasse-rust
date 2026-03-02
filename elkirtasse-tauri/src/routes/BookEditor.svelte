<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fade, slide } from 'svelte/transition';

  export let book: { id: string, name: string, author: string, betaka?: string };
  export let onSave: () => void;
  export let onCancel: () => void;

  let title = book.name;
  let author = book.author;
  let betaka = book.betaka || "";
  let isSaving = false;

  async function save() {
    isSaving = true;
    try {
      await invoke("update_book_metadata", { bookId: book.id, title, author, betaka });
      onSave();
    } catch (e) {
      console.error("Failed to update book", e);
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="fixed inset-0 z-[60] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm" transition:fade>
  <div class="bg-white dark:bg-gray-900 w-full max-w-2xl rounded-3xl shadow-2xl flex flex-col overflow-hidden" transition:slide>
    <header class="p-6 border-b border-gray-100 dark:border-gray-800 flex items-center justify-between">
      <h2 class="text-xl font-black uppercase tracking-widest text-gray-900 dark:text-white">Edit Book Metadata</h2>
      <button on:click={onCancel} class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-full transition-colors" aria-label="Cancel">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </header>

    <div class="p-8 space-y-6">
      <div class="space-y-1">
        <label for="book-title" class="text-xs font-black text-gray-400 uppercase tracking-widest px-2">Book Title</label>
        <input id="book-title" type="text" bind:value={title} class="w-full px-6 py-4 rounded-2xl bg-gray-50 dark:bg-gray-800 border-none outline-none focus:ring-2 focus:ring-blue-500 transition-all" />
      </div>

      <div class="space-y-1">
        <label for="author" class="text-xs font-black text-gray-400 uppercase tracking-widest px-2">Author</label>
        <input id="author" type="text" bind:value={author} class="w-full px-6 py-4 rounded-2xl bg-gray-50 dark:bg-gray-800 border-none outline-none focus:ring-2 focus:ring-blue-500 transition-all" />
      </div>

      <div class="space-y-1">
        <label for="betaka" class="text-xs font-black text-gray-400 uppercase tracking-widest px-2">Summary (Betaka)</label>
        <textarea id="betaka" bind:value={betaka} rows="4" class="w-full px-6 py-4 rounded-2xl bg-gray-50 dark:bg-gray-800 border-none outline-none focus:ring-2 focus:ring-blue-500 transition-all resize-none"></textarea>
      </div>
    </div>

    <footer class="p-6 bg-gray-50 dark:bg-gray-950 border-t border-gray-100 dark:border-gray-800 flex justify-end gap-3">
      <button on:click={onCancel} class="px-6 py-3 font-bold text-gray-500 hover:text-gray-900 dark:hover:text-white transition-colors">Cancel</button>
      <button
        on:click={save}
        disabled={isSaving}
        class="bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white px-10 py-3 rounded-2xl font-black shadow-lg shadow-blue-500/30 transition-all active:scale-95"
      >
        {isSaving ? "SAVING..." : "SAVE CHANGES"}
      </button>
    </footer>
  </div>
</div>
