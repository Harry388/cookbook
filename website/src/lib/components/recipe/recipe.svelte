<script lang="ts">

    import type { Recipe } from '$lib/app/recipe';

    export let recipe: Recipe;
    export let link = false;

    $: created = new Date(recipe.created);

</script>

<svelte:element this={link ? 'a' : 'div'} href="/recipe/{recipe.id}" class="card bg-base-100 shadow-xl">
    <div class="card-body">
        <h2 class="card-title">{ recipe.title }</h2>
        <a class="w-fit" href="/user/{recipe.user_id}">Posted by: { recipe.user_display_name }</a>
        <p class="w-fit">On: { created.toDateString() }</p>
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
</svelte:element>
