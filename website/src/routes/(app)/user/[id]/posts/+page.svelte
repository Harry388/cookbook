<script>

    import Posts from '$lib/components/post/posts.svelte';

    export let data;

</script>

<a href="/createpost" class="btn btn-outline">Create Post</a>

<Posts posts={data.posts} />