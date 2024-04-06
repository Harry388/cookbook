<script lang="ts">

    import { PUBLIC_BROWSER_API_URL } from "$env/static/public";

    export let width: string = '';
    export let src: string;
    export let alt: string = '';
    export let style: string = '';
    export let imageClass: string = '';

    let loaded = false;
    let error = false;

    $: url = `${PUBLIC_BROWSER_API_URL}/${src}`;

    $: {
        url;
        loaded = false;
    }

</script>

<div class="flex justify-center {width}" {style}>
    <img class={!loaded ? 'hidden' : `${imageClass} max-w-full max-h-full`} src={url} {alt} on:error={() => error = true} on:load={() => loaded = true}>
    {#if !loaded}
        {#if !$$slots.default && error}
            No Image
        {:else}
        <slot>
            <span class="loading loading-spinner loading-lg"></span>
        </slot>
        {/if}
    {/if}
</div>
