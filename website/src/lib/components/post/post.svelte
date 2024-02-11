<script lang="ts">

    import Media from '$lib/components/post/media.svelte';
    import type { Post } from '$lib/app/post';

    export let post: Post;
    export let link = false;

    $: created = new Date(post.created);

    $: media = post.media.filter(m => m != null);

</script>

<div class="card w-11/12 lg:w-1/4 bg-base-100 shadow-xl">
    {#if media.length}
        <Media media={media} />
    {/if}
    <svelte:element this={link ? 'a' : 'div'} href="/post/{post.id}" class="card-body">
        <h2 class="card-title">{ post.title }</h2>
        <a class="w-fit" href="/user/{post.user_id}">Posted by: { post.user_display_name }</a>
        {#if post.community_id != null }
            <a class="w-fit" href="/community/{post.community_id}">In Community: { post.community_title }</a>
        {/if}
        <p class="w-fit">On: { created.toDateString() }</p>
        <p>{ post.content || '' }</p>
    </svelte:element>
</div>
