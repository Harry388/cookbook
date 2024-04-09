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

<a href="/user/{data.user.id}/albums"><i class="text-lg fa-solid fa-arrow-left-long"></i></a>
{#if data.self}
    <a href="/user/{data.user.id}/albums/{data.album.id}/edit" class="btn btn-outline">Edit Album</a>
    <a href="/user/{data.user.id}/albums/{data.album.id}/add" class="btn btn-outline">Add to Album</a>
{/if}

<h3 class="text-2xl font-bold m-auto w-fit mt-5">{ data.album.title }</h3>

<div class="lg:w-5/12 lg:m-auto flex flex-col">
    {#each data.entries as entry (`${entry.type}${entry.id}`)}
        <div class="mt-5"></div>
        {#if entry.type == 'Post'} 
            <Post post={entry} link />
            {#if data.self}
                <button class="btn btn-outline btn-error w-full mt-5" on:click={() => remove(entry.id, 'post')}>Remove Post from Album</button>
            {/if}
        {:else}
            <Recipe recipe={entry} link />
            {#if data.self}
                <button class="btn btn-outline btn-error w-full mt-5" on:click={() => remove(entry.id, 'recipe')}>Remove Recipe from Album</button>
            {/if}
        {/if}
    {/each}
</div>
