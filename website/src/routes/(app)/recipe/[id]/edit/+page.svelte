<script lang="ts">

    import ListInput from '$lib/components/util/listInput.svelte';
    import Input from '$lib/components/util/input.svelte';
    import TagInput from '$lib/components/tag/tagInput.svelte';
    import { updateRecipe } from '$lib/app/recipe';
    import { invalidate } from '$app/navigation';
    import { addEntryTags, removeEntryTags } from '$lib/app/tag';
    import type { Tag } from '$lib/app/tag';

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

    async function addTag(event: CustomEvent<string>) {
        const response = await addEntryTags(data.recipe.id, [event.detail], 'recipe').run();
        if (response.ok) {
            invalidate('app:recipe');
        }
    }

    async function removeTag(event: CustomEvent<Tag>) {
        const response = await removeEntryTags(data.recipe.id, [event.detail.id], 'recipe').run();
        if (response.ok) {
            invalidate('app:recipe');
        }
    }

</script>

<div class="lg:w-1/2 m-auto">
    <h3 class="font-bold text-lg py-5">Edit Recipe</h3>
    <div class="form-control">
        <Input bind:value={title} title="Title" edit on:save={save} />
        <Input bind:value={description} title="Description" edit on:save={save} />
        <ListInput bind:list={ingredients} title="Ingredients" edit on:change={save} />
        <ListInput bind:list={method} title="Method" edit on:change={save} />
        <TagInput tags={data.tags} edit on:add={addTag} on:remove={removeTag} />
    </div>
</div>
