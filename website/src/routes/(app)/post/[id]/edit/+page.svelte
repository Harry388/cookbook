<script lang="ts">

    import { updatePost, addPostRecipe, deletePostRecipe } from '$lib/app/post';
    import { addEntryTags, removeEntryTags } from '$lib/app/tag';
    import { invalidate } from '$app/navigation';
    import RecipeComponent from '$lib/components/recipe/recipe.svelte';
    import Input from '$lib/components/util/input.svelte';
    import SelectInput from '$lib/components/util/selectInput.svelte';
    import TagInput from '$lib/components/tag/tagInput.svelte';
    import type { Tag } from '$lib/app/tag';

    export let data;

    let title = data.post.title;
    let content = data.post.content || '';
    let newRecipe: number;

    $: newRecipes = data.userRecipes.filter(ur => !data.recipes.map(r => r.id).includes(ur.id));

    async function save() {
        const response = await updatePost(data.post.id, title, content).run();
        if (response.ok) {
            invalidate('app:post');
        }
    }

    async function addRecipe() {
        const response = await addPostRecipe(data.post.id, newRecipe).run();
        if (response.ok) {
            newRecipe = -1;
            invalidate('app:post');
        }
    }

    async function deleteRecipe(recipeId: number) {
        const response = await deletePostRecipe(data.post.id, recipeId).run();
        if (response.ok) {
            invalidate('app:post');
        }
    }

    async function addTag(event: CustomEvent<string>) {
        const response = await addEntryTags(data.post.id, [event.detail], 'post').run();
        if (response.ok) {
            invalidate('app:post');
        }
    }

    async function removeTag(event: CustomEvent<Tag>) {
        const response = await removeEntryTags(data.post.id, [event.detail.id], 'post').run();
        if (response.ok) {
            invalidate('app:post');
        }
    }

</script>

<div class="flex flex-col lg:flex-row">

    <div class="flex-1 form-control">
        <h3 class="font-bold text-lg py-5">Edit Post</h3>
        <Input bind:value={title} title="Title" edit on:save={save} />
        <Input bind:value={content} title="Content" edit on:save={save} long />
        <TagInput tags={data.tags} edit on:add={addTag} on:remove={removeTag} />
    </div>

    <div class="flex-1">
        <h3 class="font-bold text-lg py-5">Attach Recipes</h3>
        <div class="flex gap-5 flex-col items-center">
            {#each data.recipes as recipe}
                <div class="flex indicator">
                    <button class="indicator-item badge badge-error text-lg" on:click={() => deleteRecipe(recipe.id)}>x</button>
                    <RecipeComponent {recipe} link />
                </div>
            {/each}
        </div>
        <label class="form-control w-full max-w-xs">
            <SelectInput bind:value={newRecipe} options={newRecipes} title="Pick Recipe" />
            <button class="btn btn-primary w-fit my-5" on:click={addRecipe}>Add Recipe</button>
            <a class="btn btn-outline w-fit" href="/recipe/create">Create New Recipe</a>
        </label>
    </div>

</div>
