<script>

    import Post from '$lib/components/post/post.svelte';
    import CreatePost from '$lib/components/post/createPost.svelte';

    export let data;

</script>

<CreatePost />

<div class="mt-5 flex gap-5">
    {#each data.posts as post}
        <Post {post} />
    {/each}
</div>