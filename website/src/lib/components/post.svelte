<script lang="ts">

    import { get } from '$lib/apiFetch';
    import type { Post } from '$lib/app/post';

    export let post: Post;

    let urls: string[] = [];

    async function getMedia() {
        for (const id of post.media) {
            const response = await get(`post/media/${id}`).run();
            const image = await response.blob();
            const url = URL.createObjectURL(image);
            urls = [...urls, url];
        }
    }

    $: {
        post;
        urls = [];
        getMedia();
    }

</script>

{ post.title }

{#if post.media.length}
    {#each urls as url}
        <img width="200px" src={url} alt="Post Image">
    {/each}
    {#if urls.length != post.media.length}
        <span class="loading loading-ring loading-md"></span>
    {/if}
{/if}