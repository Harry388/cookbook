<script lang="ts">

    import Post from '$lib/components/post/post.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';

    export let data;

    $: posts = data.entries.posts.map(p => ({...p, type: 'post'}));
    $: recipes = data.entries.recipes.map(r => ({...r, type: 'recipe'}));
    $: entries = [...posts, ...recipes];

</script>

{#if data.self}
    <a href="/user/{data.user.id}/albums/{data.album.id}/edit" class="btn btn-outline">Edit Album</a>
{/if}

<div class="w-11/12 lg:w-1/3 m-auto">
    {#each entries as entry }
        <div class="mt-5"></div>
        {#if entry.type == 'post'} 
            <Post post={entry} link />
        {:else}
            <Recipe recipe={entry} link />
        {/if}
    {/each}
</div>
