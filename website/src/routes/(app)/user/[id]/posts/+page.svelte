<script>

    import Posts from '$lib/components/post/posts.svelte';
    import CreatePost from '$lib/components/post/createPost.svelte';

    export let data;

</script>

<CreatePost />

<Posts posts={data.posts} />