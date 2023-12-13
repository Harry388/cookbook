<script lang="ts">

    import { get } from '$lib/apiFetch';
    import { onMount } from 'svelte';

    export let height: string;
    export let src: string;
    export let alt: string = '';

    let url: string | null = null;

    async function getImage() {
        const response = await get(src).run();
        const image = await response.blob();
        url = URL.createObjectURL(image);
    }

    onMount(getImage);

</script>

<div style="height: {height}" class="flex justify-center">
    {#if url}
        <img src={url} {alt} {height}>
    {:else}
        <span class="loading loading-spinner loading-lg"></span>
    {/if}
</div>