<script lang="ts">

    import RecipeComponent from '$lib/components/recipe/recipe.svelte';
    import AttachRecipeModal from '$lib/components/recipe/attachRecipeModal.svelte';
    import CreateRecipeModal from '$lib/components/recipe/createRecipeModal.svelte';
    import { getRecipe } from '$lib/app/recipe';
    import { createEventDispatcher } from 'svelte';
    import type { Recipe } from '$lib/app/recipe';

    const dispatch = createEventDispatcher();

    export let recipes: Recipe[] = [];
    export let edit = false;
    export let create = false;
    export let hideRecipes = false;

    async function addRecipe(event: CustomEvent<number>) {
        dispatch('add', event.detail);
        if (!edit) {
            const newRecipe = await getRecipe(event.detail).json();
            if (newRecipe != undefined) {
                recipes = [...recipes, newRecipe];
            }
        }
    }

    function deleteRecipe(id: number) {
        dispatch('remove', id);
        if (!edit) {
            recipes = recipes.filter(r => r.id != id);
        }
    }

</script>

{#if !hideRecipes}
    <div class="flex gap-5 flex-col items-center mb-5">
        {#each recipes as recipe}
            <div class="flex indicator w-full">
                <button class="indicator-item badge badge-error text-lg py-3" on:click={() => deleteRecipe(recipe.id)}><i class="fa-solid fa-xmark"></i></button>
                <RecipeComponent {recipe} link />
            </div>
        {/each}
    </div>
{/if}
<AttachRecipeModal on:save={addRecipe} {recipes} />
{#if create}
    <div class="divider">OR</div>
    <CreateRecipeModal on:save={addRecipe} />
{/if}
