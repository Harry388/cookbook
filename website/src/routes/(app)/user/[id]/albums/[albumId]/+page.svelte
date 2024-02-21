<script lang="ts">

    import Post from '$lib/components/post/post.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';
    import { removeAlbumEntry } from '$lib/app/album';
    import { invalidate } from '$app/navigation';

    export let data;

    async function remove(id: number, type: 'post' | 'recipe') {
        const response = await removeAlbumEntry(data.album.id, id, type).run();
        if (response.ok) {
            invalidate('app:album');
        }
    }

</script>

{#if data.self}
    <a href="/user/{data.user.id}/albums/{data.album.id}/edit" class="btn btn-outline">Edit Album</a>
    <a href="/user/{data.user.id}/albums/{data.album.id}/add" class="btn btn-outline">Add to Album</a>
{/if}

<div class="w-11/12 lg:w-1/3 m-auto">
    {#each data.entries as entry }
        <div class="mt-5"></div>
        {#if entry.type == 'Post'} 
            <Post post={entry} link />
            {#if data.self}
                <button class="btn btn-outline" on:click={() => remove(entry.id, 'post')}>Remove</button>
            {/if}
        {:else}
            <Recipe recipe={entry} link />
            {#if data.self}
                <button class="btn btn-outline" on:click={() => remove(entry.id, 'recipe')}>Remove</button>
            {/if}
        {/if}
    {/each}
</div>
