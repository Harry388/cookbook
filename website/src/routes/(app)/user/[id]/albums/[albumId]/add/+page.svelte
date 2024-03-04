<script lang="ts">

    import Post from '$lib/components/post/post.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';
    import { addAlbumEntry } from '$lib/app/album';
    import { invalidate } from '$app/navigation';

    export let data;

    $: includedPosts = data.entries.filter(e => e.type == 'Post').map(p => p.id);
    $: includedRecipes = data.entries.filter(e => e.type == 'Recipe').map(r => r.id);
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

<a href="/user/{data.user.id}/albums/{data.album.id}"><i class="text-lg fa-solid fa-arrow-left-long"></i></a>

<div class="lg:w-5/12 lg:m-auto flex flex-col">
    {#each entries as entry }
        <div class="mt-5"></div>
        <div class="flex gap-x-5">
            {#if entry.type == 'post'} 
                <Post post={entry} link />
                <button class="fa-regular fa-plus text-2xl btn" on:click={() => add(entry.id, 'post')}></button>
            {:else}
                <Recipe recipe={entry} link />
                <button class="fa-regular fa-plus text-2xl btn" on:click={() => add(entry.id, 'recipe')}></button>
            {/if}
        </div>
    {/each}
</div>
