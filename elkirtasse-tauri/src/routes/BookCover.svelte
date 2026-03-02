<script lang="ts">
  export let title: string;
  export let author: string;
  export let progress: number | undefined = 0;

  const colors = [
    'bg-blue-600', 'bg-indigo-600', 'bg-purple-600', 'bg-teal-600', 'bg-emerald-600', 'bg-cyan-600'
  ];

  // Deterministic color based on title length
  const bgColor = colors[title.length % colors.length];
</script>

<div class="relative w-full aspect-[2/3] rounded-2xl overflow-hidden shadow-xl group-hover:scale-105 transition-transform duration-500">
  <div class="absolute inset-0 {bgColor} flex flex-col p-6 text-white justify-between">
    <div class="space-y-4">
      <div class="w-10 h-1 bg-white/30 rounded-full"></div>
      <h3 class="text-xl font-black leading-tight tracking-tighter" dir="rtl">{title}</h3>
    </div>

    <div class="space-y-4">
      <p class="text-xs font-bold uppercase tracking-widest text-white/60">{author}</p>
      {#if progress !== undefined && progress > 0}
        <div class="flex items-center gap-2">
           <div class="flex-1 h-1 bg-white/20 rounded-full overflow-hidden">
             <div class="h-full bg-white" style="width: {progress * 100}%"></div>
           </div>
           <span class="text-[10px] font-black">{Math.round(progress * 100)}%</span>
        </div>
      {/if}
    </div>
  </div>

  <!-- Texture Overlay -->
  <div class="absolute inset-0 bg-gradient-to-tr from-black/20 to-transparent pointer-events-none"></div>
  <div class="absolute inset-y-0 left-0 w-4 bg-black/10 shadow-inner pointer-events-none"></div>
</div>
