<script lang="ts">

    import Post from '$lib/components/post/post.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';
    import { addAlbumEntry } from '$lib/app/album';
    import { invalidate } from '$app/navigation';

    export let data;

    $: includedPosts = data.entries.posts.map(p => p.id);
    $: includedRecipes = data.entries.recipes.map(r => r.id);
    $: posts = data.posts.map(p => ({...p, type: 'post'})).filter(p => !includedPosts.includes(p.id));
    $: recipes = data.recipes.map(r => ({...r, type: 'recipe'})).filter(r => !includedRecipes.includes(r.id));
    $: entries = [...posts, ...recipes];

    async function add(id: number, type: 'post' | 'recipe') {
        const response = await addAlbumEntry(data.album.id, id, type).run();
        if (response.ok) {
            invalidate('app:album');
        }
    }

</script>

<a href="/user/{data.user.id}/albums/{data.album.id}" class="btn btn-outline">Back</a>

<div class="w-11/12 lg:w-1/3 m-auto">
    {#each entries as entry }
        <div class="mt-5"></div>
        {#if entry.type == 'post'} 
            <Post post={entry} link />
            <button class="btn btn-outline" on:click={() => add(entry.id, 'post')}>Add</button>
        {:else}
            <Recipe recipe={entry} link />
            <button class="btn btn-outline" on:click={() => add(entry.id, 'recipe')}>Add</button>
        {/if}
    {/each}
</div>
