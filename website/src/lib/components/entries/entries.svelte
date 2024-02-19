<script lang="ts">

    import Post from '$lib/components/post/post.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';
    import type { Entries } from '$lib/app/album';

    export let entries: Entries;

    $: posts = entries.posts.map(p => ({...p, type: 'post'}));
    $: recipes = entries.recipes.map(r => ({...r, type: 'recipe'}));
    $: mappedEntries = [...posts, ...recipes];

</script>

{#each mappedEntries as entry }
    <div class="mt-5"></div>
    {#if entry.type == 'post'} 
        <Post post={entry} link />
    {:else}
        <Recipe recipe={entry} link />
    {/if}
{/each}
