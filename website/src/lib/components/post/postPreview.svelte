<script lang="ts">

    import Media from '$lib/components/post/media.svelte';
    import type { Post } from '$lib/app/post';

    export let post: Post;
    export let link = false;

</script>

<div class="card w-80 lg:w-1/4 bg-base-100 shadow-xl">
    {#if post.media.length}
        <Media media={post.media} />
    {/if}
    <svelte:element this={link ? 'a' : 'div'} href="/post/{post.id}" class="card-body">
        <h2 class="card-title">{ post.title }</h2>
        <p>{ post.content || '' }</p>
    </svelte:element>
</div>
