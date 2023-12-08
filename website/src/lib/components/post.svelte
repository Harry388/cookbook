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

<div class="card w-96 bg-base-100 shadow-xl">
    {#if post.media.length}
        {#each post.media as _, i}
            <figure class="h-30">
                {#if urls[i]}
                    <img src={urls[i]} alt="Post Image">
                {:else}
                    <span class="loading loading-ring loading-md"></span>
                {/if}
            </figure>
        {/each}
    {/if}
    <div class="card-body">
        <h2 class="card-title">{ post.title }</h2>
        <p>{ post.content }</p>
    </div>
</div>
