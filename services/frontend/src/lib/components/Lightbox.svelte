<script lang="ts">
  export let src: string;
  export let alt: string;
  export let open: boolean;
  export let onClose: () => void;
</script>

{#if open}
  <div
    class="fixed inset-0 bg-black/90 flex items-center justify-center z-[1000] transition-opacity duration-300"
    on:click={onClose}
  >
    <div class="relative">
      <img
        src={src}
        alt={alt}
        class="max-w-[90vw] max-h-[90vh] object-contain rounded-lg shadow-2xl"
        on:click|stopPropagation
      />
      <button
        class="absolute -top-3 -right-3 text-white text-2xl font-bold bg-black/70 hover:bg-black/90 rounded-full p-2 shadow-lg transition"
        on:click={onClose}
        aria-label="Close"
      >
        ✕
      </button>
    </div>
  </div>
{/if}
