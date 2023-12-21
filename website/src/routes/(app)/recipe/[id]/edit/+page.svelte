<script lang="ts">

    import ListInput from '$lib/components/util/listInput.svelte';
    import { updateRecipe } from '$lib/app/recipe';

    export let data;

    let title = data.recipe.title;
    let description = data.recipe.description;
    let ingredients = data.recipe.ingredients;
    let method = data.recipe.method;

    async function save() {
        const response = await updateRecipe(data.recipe.id, title, description, ingredients, method);
        if (response.ok) {
            history.back();
        }
    }

</script>

<h3 class="font-bold text-lg py-5">Edit Recipe</h3>
<div class="form-control">
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Title</span>
    </label>
    <input type="text" min="1" bind:value={title} placeholder="Title" class="input input-bordered" />
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Description</span>
    </label>
    <textarea class="textarea textarea-bordered" placeholder="Description" bind:value={description}></textarea>
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