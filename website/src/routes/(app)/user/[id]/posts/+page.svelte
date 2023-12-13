<script>

    import Posts from '$lib/components/post/posts.svelte';

    export let data;

</script>

{#if data.self}
    <a href="/createpost" class="btn btn-outline">Create Post</a>
{/if}

<Posts posts={data.posts} />