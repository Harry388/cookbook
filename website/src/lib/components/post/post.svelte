<script lang="ts">

    import Media from '$lib/components/post/media.svelte';
    import type { PostFull } from '$lib/app/post';

    export let post: PostFull;
    export let link = false;

</script>

<div class="card w-11/12 lg:w-1/4 bg-base-100 shadow-xl">
    {#if post.media.length}
        <Media media={post.media} />
    {/if}
    <svelte:element this={link ? 'a' : 'div'} href="/post/{post.id}" class="card-body">
        <h2 class="card-title">{ post.title }</h2>
        <a class="w-fit" href="/user/{post.user_id}">Posted by: { post.user.display_name }</a>
        {#if post.community }
            <a class="w-fit" href="/community/{post.community_id}">In Community: { post.community.title }</a>
        {/if}
        <p>{ post.content || '' }</p>
    </svelte:element>
</div>
