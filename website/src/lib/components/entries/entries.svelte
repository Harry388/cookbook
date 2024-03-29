<script lang="ts">

    import Post from '$lib/components/post/post.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';
    import type { Entry } from '$lib/app/entry';

    export let entries: Entry[];

</script>

{#each entries as entry (`${entry.type}${entry.id}`)}
    <div class="mt-5"></div>
    {#if entry.type == 'Post'} 
        <Post post={entry} link />
    {:else}
        <Recipe recipe={entry} link />
    {/if}
{/each}
