<script lang="ts">

    import ListInput from '$lib/components/util/listInput.svelte';
    import Input from '$lib/components/util/input.svelte';
    import { updateRecipe } from '$lib/app/recipe';
    import { invalidate } from '$app/navigation';

    export let data;

    let title = data.recipe.title;
    let description = data.recipe.description || '';
    let ingredients = data.recipe.ingredients;
    let method = data.recipe.method;

    async function save() {
        const response = await updateRecipe(data.recipe.id, title, description, ingredients, method).run();
        if (response.ok) {
            invalidate('app:recipe');
        }
    }

</script>

<h3 class="font-bold text-lg py-5">Edit Recipe</h3>
<div class="form-control">
    <Input bind:value={title} title="Title" edit on:save={save} />
    <Input bind:value={description} title="Description" edit on:save={save} />
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Ingredients</span>
    </label>
    <ListInput bind:list={ingredients} />
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Method</span>
    </label>
    <ListInput bind:list={method} />
    <button class="btn btn-primary w-fit mt-5" on:click={save}>Save</button>
</div>
