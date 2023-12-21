<script lang="ts">

    import type { Recipe } from '$lib/app/recipe';

    export let recipe: Recipe;

</script>

<a href="/recipe/{recipe.id}" class="card w-11/12 lg:w-96 bg-base-100 shadow-xl">
    <div class="card-body">
        <h2 class="card-title">{ recipe.title }</h2>
        <p>{ recipe.description || '' }</p>
        <h3 class="font-bold text-lg py-5">Ingredients</h3>
        {#each recipe.ingredients as ingredient, i}
            <p>{i + 1}. {ingredient}</p>
        {/each}
        <h3 class="font-bold text-lg py-5">Method</h3>
        {#each recipe.method as step, i}
            <p>{i + 1}. {step}</p>
        {/each}
    </div>
</a>
