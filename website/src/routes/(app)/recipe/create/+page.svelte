<script lang="ts">

    import ListInput from '$lib/components/util/listInput.svelte';
    import { createRecipe } from '$lib/app/recipe';

    let title = '';
    let description = '';
    let ingredients: string[];
    let method: string[];

    async function create() {
        const response = await createRecipe(title, description, ingredients, method).run();
        if (response.ok) {
            history.back();
        }
    }

</script>

<h3 class="font-bold text-lg py-5">Create a Recipe</h3>
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
    <button class="btn btn-primary w-fit mt-5" on:click={create}>Create</button>
</div>
