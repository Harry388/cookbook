<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';

    export let data;

</script>

<Posts posts={data.posts} />
