<script>
    import diagram from '$lib/assets/flowchart.png';
    import oauth from '$lib/assets/oauth_flow.png';
    import ezchart from '$lib/assets/ezchart.png';
    let diagramOpen = false;
    let oauthOpen = false;
    let ezchartOpen = false;
</script>

<h1 id="ezchart">Overview</h1>
<img src={ezchart} alt="Simple Chart" onclick={()=> ezchartOpen = true}/>
{#if ezchartOpen}
    <!-- Lightbox overlay -->
    <div class="overlay" onclick={() => ezchartOpen = false}>
        <img
        src={ezchart}
        alt="Full view"
        class="lightbox"
        onclick={(e) => e.stopPropagation()}
        />
        <button class="close" onclick={() => ezchartOpen = false}>✕</button>
    </div>
{/if}
<p class="spacer">
    This diagram shows the simplified function of the Discord Authenticator. Click to enlarge.
</p>

<h1 id="diagram">Discord Authenticator Flow</h1>
<img src={diagram} alt="Diagram" onclick={()=> diagramOpen = true}/>
{#if diagramOpen}
    <!-- Lightbox overlay -->
    <div class="overlay" onclick={() => diagramOpen = false}>
        <img
        src={diagram}
        alt="Full view"
        class="lightbox"
        onclick={(e) => e.stopPropagation()}
        />
        <button class="close" onclick={() => diagramOpen = false}>✕</button>
    </div>
{/if}
<p class="spacer">
    This diagram shows the sequence of operations of the Discord Authenticator. Click to enlarge.
</p>

<h1 id="oauth">OAuth2 Flow</h1>
<img src={oauth} alt="Oauth2 flow" onclick={()=> oauthOpen = true}/>
{#if oauthOpen}
    <!-- Lightbox overlay -->
    <div class="overlay" onclick={() => oauthOpen = false}>
        <img
        src={oauth}
        alt="Full view"
        class="lightbox"
        onclick={(e) => e.stopPropagation()}
        />
        <button class="close" onclick={() => open = false}>✕</button>
    </div>
{/if}
<p class="spacer">
    This diagram shows the OAuth2 flow used by the Discord Authenticator. Click to enlarge.
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
.spacer {
    margin-bottom: 2rem;
}
</style>