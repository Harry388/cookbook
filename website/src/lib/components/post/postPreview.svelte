<script lang="ts">

    import Media from '$lib/components/post/media.svelte';
    import { createEventDispatcher } from 'svelte';
    import type { Post } from '$lib/app/post';

	const dispatch = createEventDispatcher();

    export let post: Post;

</script>

<div class="card w-80 lg:w-1/4 bg-base-100 shadow-xl">
    {#if post.media.length}
        {#each post.media as id}
            <Media {id} />
        {/each}
    {/if}
    <div class="card-body">
        <button class="btn btn-outline" on:click={() => dispatch('delete')}>Delete</button>
        <h2 class="card-title">{ post.title }</h2>
        <p>{ post.content || '' }</p>
    </div>
</div>
