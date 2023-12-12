<script lang="ts">

    import Media from '$lib/components/post/media.svelte';
    import type { Post } from '$lib/app/post';

    export let post: Post;

</script>

<div class="card w-96 bg-base-100 shadow-xl">
    {#if post.media.length}
        {#each post.media as id}
            <Media {id} />
        {/each}
    {/if}
    <div class="card-body">
        <h2 class="card-title">{ post.title }</h2>
        <p>{ post.content || '' }</p>
    </div>
</div>
