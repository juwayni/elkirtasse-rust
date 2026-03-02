<script lang="ts">
  import { fade, fly } from 'svelte/transition';

  export let onClose: () => void;

  const sections = [
    {
      title: "Getting Started",
      content: "Elkirtasse Modern is your digital library for Islamic literature. You can browse books by category, search through the entire collection, and read with specialized Arabic typography."
    },
    {
      title: "Importing Books",
      content: "Go to the Library view and use the 'Batch Import' (folder icon) to scan a directory for PDF, ePub, Docx, and .bok files. Use the 'Import Book' (plus icon) for single files."
    },
    {
      title: "Searching",
      content: "The Search tab uses a high-performance engine. You can search by text, author, or book title. After importing new books, use 'Rebuild Search Index' on the Dashboard to ensure they are searchable."
    },
    {
      title: "Reading Experience",
      content: "In the Reader, you can navigate using the Table of Contents, set custom font sizes, and toggle Dark Mode. Your progress is saved automatically as you read."
    },
    {
      title: "Annotations & Bookmarks",
      content: "Select text while reading to add a highlight or note. Use the bookmark icon in the header to save your favorite pages. You can export all your annotations to Markdown from the Dashboard."
    },
    {
      title: "Keyboard Shortcuts",
      content: `
        - **Enter**: Confirm search / Open book
        - **Left/Right Arrows**: Previous/Next page in Reader
        - **ESC**: Close modals
        - **Ctrl+F**: Focus search input
      `
    }
  ];
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md" transition:fade>
  <div class="bg-white dark:bg-gray-900 w-full max-w-3xl max-h-[85vh] rounded-[2.5rem] shadow-2xl flex flex-col overflow-hidden border border-gray-100 dark:border-gray-800" in:fly={{ y: 20 }}>
    <header class="p-8 border-b border-gray-50 dark:border-gray-800 flex items-center justify-between">
      <div>
        <h2 class="text-xs font-black tracking-[0.3em] text-blue-600 uppercase mb-2">Help & Documentation</h2>
        <h3 class="text-3xl font-black text-gray-900 dark:text-white tracking-tighter">User Guide</h3>
      </div>
      <button on:click={onClose} class="p-3 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-2xl transition-all active:scale-90" aria-label="Close">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </header>

    <div class="flex-1 overflow-y-auto p-10 space-y-12">
      {#each sections as section}
        <section>
          <h4 class="text-sm font-black uppercase tracking-widest text-gray-400 mb-4 flex items-center gap-3">
             <span class="w-6 h-px bg-blue-600"></span> {section.title}
          </h4>
          <div class="text-lg text-gray-600 dark:text-gray-300 leading-relaxed prose prose-blue dark:prose-invert max-w-none">
             {#if section.title === "Keyboard Shortcuts"}
                <pre class="bg-gray-50 dark:bg-gray-950 p-6 rounded-2xl border border-gray-100 dark:border-gray-800 font-mono text-sm">{section.content.trim()}</pre>
             {:else}
                <p>{section.content}</p>
             {/if}
          </div>
        </section>
      {/each}
    </div>

    <footer class="p-8 bg-gray-50 dark:bg-gray-950 border-t border-gray-100 dark:border-gray-800 text-center">
        <p class="text-xs font-bold text-gray-400 uppercase tracking-widest">Elkirtasse Modern v1.0.0 Stable</p>
    </footer>
  </div>
</div>
