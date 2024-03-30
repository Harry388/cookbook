<script lang="ts">

    import type { Recipe } from '$lib/app/recipe';

    export let recipe: Recipe;

</script>

<div class="flex flex-col w-fit m-auto gap-3">
    <h1 class="font-bold text-3xl">{ recipe.title }</h1>
    <p>{ recipe.description }</p>
    <h2 class="font-bold text-2xl">Ingredients</h2>
    {#each recipe.ingredients as ingredient, i}
        <div>{ i + 1}. { ingredient }</div>
    {/each}
    <h2 class="font-bold text-2xl">Method</h2>
    {#each recipe.method as step, i}
        <div>{ i + 1}. { step }</div>
    {/each}
</div>
