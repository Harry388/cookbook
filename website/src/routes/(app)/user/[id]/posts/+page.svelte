<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';

    export let data;

</script>

{#if data.self}
    <a href="/post/create" class="btn btn-outline">Create Post</a>
{/if}

<Posts posts={data.posts} />
