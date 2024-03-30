<script lang="ts">

    import CreateRecipe from '$lib/components/recipe/createRecipe.svelte';
    import { createRecipe } from '$lib/app/recipe';
    import type { Tag } from '$lib/app/tag';

    let title = '';
    let description = '';
    let ingredients: string[];
    let method: string[];
    let tags: Tag[];

    async function create() {
        const t = tags.map(t => t.tag);
        const response = await createRecipe(title, description, ingredients, method, t).run();
        if (response.ok) {
            history.back();
        }
    }

</script>

<div class="lg:w-1/2 m-auto">
    <h3 class="font-bold text-lg py-5">Create a Recipe</h3>
    <CreateRecipe bind:title={title} bind:description={description} bind:ingredients={ingredients} bind:method={method} bind:tags={tags} />
    <button class="btn btn-success btn-outline w-full mt-5" on:click={create}>Create</button>
</div>
