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

<h3 class="font-bold text-lg py-5">Create a Recipe</h3>
<div class="form-control">
    <Input bind:value={title} title="Title" />
    <Input bind:value={description} title="Description" long />
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
    <TagInput bind:tags={tags} />
    <button class="btn btn-primary w-fit mt-5" on:click={create}>Create</button>
</div>
