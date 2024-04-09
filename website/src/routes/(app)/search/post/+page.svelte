<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';

    export let data;

</script>

{#if data.posts.length == 0}
    <p class="text-lg text-center">No posts found</p>
{/if}
<Posts posts={data.posts} />
