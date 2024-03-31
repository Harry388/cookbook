<script lang="ts">

    import AttachRecipe from '$lib/components/recipe/attachRecipe.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';
    import Confirm from '$lib/components/util/confirm.svelte';
    import EditImage from '$lib/components/util/editImage.svelte';
    import { addCookbookRecipe, removeCookbookSection, removeCookbookRecipe, setCookbookRecipePic } from '$lib/app/cookbook';
    import { createEventDispatcher } from 'svelte';
    import type { BookSection } from '$lib/app/page';

    const dispatch = createEventDispatcher();

    export let cookbookId: number;
    export let section: BookSection;

    async function removeSection() {
        const response = await removeCookbookSection(cookbookId, section.section.id).run();
        if (response.ok) {
            dispatch('change');
        }
    }

    async function addRecipe(event: CustomEvent<number>) {
        const response = await addCookbookRecipe(cookbookId, section.section.id, event.detail).run();
        if (response.ok) {
            dispatch('change');
        }
    }

    async function removeRecipe(id: number) {
        const response = await removeCookbookRecipe(cookbookId, section.section.id, id).run();
        if (response.ok) {
            dispatch('change');
        }
    }

    async function addRecipePic(pic: File, id: number, after: Function) {
        const response = await setCookbookRecipePic(cookbookId, section.section.id, id, pic).run();
        if (response.ok) {
            dispatch('change');
            after();
        }
    }

</script>

<h2 class="font-bold text-2xl">{ section.section.title }</h2>
<Confirm let:show on:confirm={removeSection}>
    <button on:click={show} class="btn btn-error">Delete Section</button>
</Confirm>
<div class="flex gap-5 flex-col items-center">
    {#each section.recipes as recipe (recipe.id)}
        <div class="flex w-full">
            <div class="flex indicator w-1/2">
                <button class="indicator-item badge badge-error text-lg py-3" on:click={() => removeRecipe(recipe.id)}><i class="fa-solid fa-xmark"></i></button>
                <Recipe {recipe} link />
            </div>
            <div class="divider divider-horizontal"></div>
            <div class="w-1/2">
                <EditImage src="cookbook/{cookbookId}/section/{section.section.id}/recipe/{recipe.id}/image" on:change={e => addRecipePic(e.detail.file, recipe.id, e.detail.after)} />
            </div>
        </div>
    {/each}
</div>
<AttachRecipe recipes={section.recipes} edit on:add={addRecipe} hideRecipes />
