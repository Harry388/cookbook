<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';

    export let data;

</script>

{#if data.self}
    <a href="/post/create" class="btn btn-outline">Create Post</a>
{/if}

<div class="w-11/12 lg:w-1/3 m-auto">
    <Posts posts={data.posts} />
</div>
