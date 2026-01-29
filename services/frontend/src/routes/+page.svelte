<script lang="ts">
  import Lightbox from '$lib/components/Lightbox.svelte';
  import diagram from '$lib/assets/flowchart.png';
  import oauth from '$lib/assets/oauth_flow.png';
  import ezchart from '$lib/assets/ezchart.png';

  type ImageData = { id: string; src: string; alt: string; title: string; description: string };
  const images: ImageData[] = [
    {
      id: 'ezchart',
      src: ezchart,
      alt: 'Simple Chart',
      title: 'Overview',
      description: 'This diagram shows the simplified function of the Discord Authenticator. Click to enlarge.'
    },
    {
      id: 'diagram',
      src: diagram,
      alt: 'Diagram',
      title: 'Discord Authenticator Flow',
      description: 'This diagram shows the sequence of operations of the Discord Authenticator. Click to enlarge.'
    },
    {
      id: 'oauth',
      src: oauth,
      alt: 'Oauth2 flow',
      title: 'OAuth2 Flow',
      description: 'This diagram shows the OAuth2 flow used by the Discord Authenticator. Click to enlarge.'
    }
  ];

  let openId: string | null = null;
</script>

<div class="space-y-16 px-4 md:px-16 py-8">
  {#each images as image}
    <section id={image.id} class="space-y-4 scroll-mt-40">
      <h1 class="text-3xl md:text-4xl font-bold text-gray-900 dark:text-gray-100 transition-colors duration-500">
        {image.title}
      </h1>
      <img
        src={image.src}
        alt={image.alt}
        on:click={() => openId = image.id}
        class="cursor-pointer rounded-lg shadow-lg hover:shadow-2xl transition-transform duration-300 hover:scale-[1.02] mx-auto"
      />
      <Lightbox
        src={image.src}
        alt={image.alt}
        open={openId === image.id}
        onClose={() => openId = null}
      />
      <p class="text-gray-700 dark:text-gray-300 text-lg leading-relaxed transition-colors duration-500 max-w-3xl mx-auto">
        {image.description}
      </p>
    </section>
  {/each}
</div>
