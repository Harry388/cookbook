<script lang="ts">

    import { PUBLIC_BROWSER_API_URL } from "$env/static/public";

    export let height: string = '';
    export let src: string;
    export let alt: string = '';

    let loaded = false;

    $: url = `${PUBLIC_BROWSER_API_URL}/${src}`;

    $: {
        url;
        loaded = false;
    }

</script>

<div style="height: {height}" class="flex justify-center">
    <img class={!loaded ? 'hidden' : ''} src={url} {alt} {height} on:load={() => loaded = true}>
    {#if !loaded}
        <slot>
            <span class="loading loading-spinner loading-lg"></span>
        </slot>
    {/if}
</div>