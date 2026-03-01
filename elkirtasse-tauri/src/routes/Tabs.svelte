<script lang="ts">
  export let tabs: { id: string, title: string }[] = [];
  export let activeTabId: string;
  export let onSwitch: (id: string) => void;
  export let onClose: (id: string) => void;
</script>

<div class="flex items-center gap-1 px-4 pt-2 bg-gray-100 dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800 overflow-x-auto no-scrollbar">
  {#each tabs as tab}
    <div
      class="group flex items-center gap-2 px-4 py-2 rounded-t-xl text-sm font-bold transition-all cursor-pointer min-w-[120px] max-w-[240px] {activeTabId === tab.id ? 'bg-white dark:bg-gray-950 text-blue-600 dark:text-blue-400' : 'text-gray-500 hover:bg-gray-200 dark:hover:bg-gray-800'}"
      on:click={() => onSwitch(tab.id)}
    >
      <span class="truncate flex-1">{tab.title}</span>
      <button
        class="opacity-0 group-hover:opacity-100 p-0.5 hover:bg-gray-300 dark:hover:bg-gray-700 rounded-full transition-all"
        on:click|stopPropagation={() => onClose(tab.id)}
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  {/each}
</div>

<style>
  .no-scrollbar::-webkit-scrollbar {
    display: none;
  }
  .no-scrollbar {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
</style>
