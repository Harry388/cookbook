<script lang="ts">

    import RecipeComponent from '$lib/components/recipe/recipe.svelte';
    import { addCookbookRecipe, removeCookbookSection, removeCookbookRecipe } from '$lib/app/cookbook';
    import { createEventDispatcher } from 'svelte';
    import type { BookSection } from '$lib/app/page';
    import type { Recipe } from '$lib/app/recipe';

    const dispatch = createEventDispatcher();

    export let cookbookId: number;
    export let section: BookSection;
    export let userRecipes: Recipe[];

    let newRecipe: number;

    $: newRecipes = userRecipes.filter(r => !section.recipes.map(sr => sr.id).includes(r.id));

    async function removeSection() {
        if (!confirm('Are you sure?')) return;
        const response = await removeCookbookSection(cookbookId, section.section.id).run();
        if (response.ok) {
            dispatch('change');
        }
    }

    async function addRecipe() {
        const response = await addCookbookRecipe(cookbookId, section.section.id, newRecipe).run();
        if (response.ok) {
            newRecipe = -1;
            dispatch('change');
        }
    }

    async function removeRecipe(id: number) {
        const response = await removeCookbookRecipe(cookbookId, section.section.id, id).run();
        if (response.ok) {
            dispatch('change');
        }
    }

</script>

<h2 class="font-bold text-2xl">{ section.section.title }</h2>
<button on:click={removeSection} class="btn btn-error">Delete Section</button>
{#each section.recipes as recipe}
    <RecipeComponent {recipe} link />
    <button on:click={() => removeRecipe(recipe.id)} class="btn btn-error">Remove Recipe</button>
{/each}

<label class="form-control w-full max-w-xs">
    <div class="label">
        <span class="label-text">Pick Recipe</span>
    </div>
    <select bind:value={newRecipe} class="select select-bordered">
        <option value={-1} selected>Pick one</option>
        {#each newRecipes as recipe}
            <option value={recipe.id}>{ recipe.title }</option>
        {/each}
    </select>
    <button class="btn btn-primary w-fit my-5" on:click={addRecipe}>Add Recipe</button>
</label>
