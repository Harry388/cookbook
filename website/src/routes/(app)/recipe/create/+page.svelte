<script lang="ts">

    import ListInput from '$lib/components/util/listInput.svelte';
    import TagInput from '$lib/components/tag/tagInput.svelte';
    import Input from '$lib/components/util/input.svelte';
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
    <div class="form-control">
        <Input bind:value={title} title="Title" />
        <Input bind:value={description} title="Description" long />
        <ListInput bind:list={ingredients} title="Ingredients" />
        <ListInput bind:list={method} title="Method" />
        <TagInput bind:tags={tags} />
        <button class="btn btn-primary w-fit mt-5" on:click={create}>Create</button>
    </div>
</div>
