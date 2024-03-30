<script lang="ts">

    import SelectInput from '$lib/components/util/selectInput.svelte';
    import RecipeComponent from '$lib/components/recipe/recipe.svelte';
    import { getUserRecipes } from '$lib/app/recipe';
    import { createEventDispatcher, onMount, getContext } from 'svelte';
    import type { Recipe } from '$lib/app/recipe';

    const dispatch = createEventDispatcher();

    const id: number = getContext('id');

    export let recipes: Recipe[] = [];
    export let edit = false;
    export let create = false;

    let newRecipeId: number;
    let options: Recipe[] = [];

    $: newRecipes = options.filter(r => !recipes.map(rr => rr.id).includes(r.id));

    function addRecipe() {
        dispatch('add', newRecipeId);
        if (!edit) {
            const newRecipe = options.find(r => r.id == newRecipeId);
            if (newRecipe != undefined) {
                recipes = [...recipes, newRecipe];
            }
        }
        newRecipeId = -1;
    }

    function deleteRecipe(id: number) {
        dispatch('remove', id);
        if (!edit) {
            recipes = recipes.filter(r => r.id != id);
        }
    }

    onMount(async () => {
        options = await getUserRecipes(id).json();
    });

</script>

<div class="flex gap-5 flex-col items-center">
    {#each recipes as recipe}
        <div class="flex indicator">
            <button class="indicator-item badge badge-error text-lg" on:click={() => deleteRecipe(recipe.id)}>x</button>
            <RecipeComponent {recipe} link />
        </div>
    {/each}
</div>
<label class="form-control w-full max-w-xs">
    <SelectInput bind:value={newRecipeId} options={newRecipes} title="Pick Recipe" />
    <button class="btn btn-primary w-fit my-5" on:click={addRecipe}>Add Recipe</button>
    {#if create}
        <a class="btn btn-outline w-fit" href="/recipe/create">Create New Recipe</a>
    {/if}
</label>
