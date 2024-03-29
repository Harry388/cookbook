<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';

    export let data;

</script>

<div class="lg:w-1/3 lg:m-auto flex flex-col">
    {#if data.self}
        <a href="/post/create" class="btn btn-outline mb-5"><i class="fa-solid fa-plus"></i>Create Post</a>
    {/if}
    <Posts posts={data.posts} />
</div>
