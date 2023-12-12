<script lang="ts">

    import { get } from '$lib/apiFetch';
    import { onMount } from 'svelte';

    export let id: number;

    const height = '250px';

    let url: string | null = null;

    async function getMedia() {
        const response = await get(`post/media/${id}`).run();
        const image = await response.blob();
        url = URL.createObjectURL(image);
    }

    onMount(getMedia);

</script>

<div style="height: {height}" class="flex justify-center">
    {#if url}
        <img src={url} alt="Post Image" {height}>
    {:else}
        <span class="loading loading-spinner loading-lg"></span>
    {/if}
</div>