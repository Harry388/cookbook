<script lang="ts">

    import Entry from '$lib/components/entries/entry.svelte';
    import type { Recipe } from '$lib/app/recipe';

    export let recipe: Recipe;
    export let link = false;

</script>

<Entry entry={recipe} {link} type="recipe">
    {#if recipe.ingredients.length > 0}
        <h3 class="font-bold text-lg">Ingredients</h3>
        {#each recipe.ingredients as ingredient, i}
            <p class="text-lg">{i + 1}. {ingredient}</p>
        {/each}
    {/if}
    {#if recipe.method.length > 0}
        <h3 class="font-bold text-lg">Method</h3>
        {#each recipe.method as step, i}
            <p class="text-lg">{i + 1}. {step}</p>
        {/each}
    {/if}
</Entry>
