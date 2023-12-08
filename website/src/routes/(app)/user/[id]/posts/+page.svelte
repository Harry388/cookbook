<script lang="ts">

    import { get } from '$lib/apiFetch';
    import Post from '$lib/components/post.svelte';

    export let data;

</script>

{#each data.posts as post}
    <Post {post} />
{/each}