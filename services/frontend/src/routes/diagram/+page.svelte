<script>
    import diagram from '$lib/assets/flowchart.png';
	import { stopPropagation } from 'svelte/legacy';
    let open = false;
</script>

<img src={diagram} alt="Diagram" onclick={()=> open = true}/>
{#if open}
    <!-- Lightbox overlay -->
    <div class="overlay" onclick={() => open = false}>
        <img
        src={diagram}
        alt="Full view"
        class="lightbox"
        onclick={(e) => e.stopPropagation()}
        />
        <button class="close" onclick={() => open = false}>✕</button>
    </div>
{/if}
<p>
    This diagram shows the sequence of operations of the Discord Authenticator. Click to enlarge.
</p>

<style>
    .overlay {
        position: fixed;
        inset: 0;
        background: rgba(0,0,0,0.85);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }


    .lightbox {
        max-width: 90vw;
        max-height: 90vh;
        object-fit: contain;
    }


.close {
        position: absolute;
        top: 20px;
        right: 20px;
        font-size: 2rem;
        background: none;
        border: none;
        color: white;
        cursor: pointer;
    }
</style>