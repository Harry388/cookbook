<script lang="ts">

    import { updatePost, addPostRecipe, getPostRecipes, deletePostRecipe } from '$lib/app/post';
    import RecipeComponent from '$lib/components/recipe/recipe.svelte';
    import type { Recipe } from '$lib/app/recipe';

    export let data;

    let title = data.post.title;
    let content = data.post.content;
    let newRecipe: number;
    let recipes = data.recipes;

    $: newRecipes = data.userRecipes.filter(ur => !recipes.map(r => r.id).includes(ur.id));

    async function save() {
        const response = await updatePost(data.post.id, title, content);
        if (response.ok) {
            history.back();
        }
    }

    async function addRecipe() {
        const response = await addPostRecipe(data.post.id, newRecipe);
        newRecipe = -1;
        if (response.ok) {
            recipes = await getPostRecipes(data.post.id);
        }
    }

    async function deleteRecipe(recipeId: number) {
        const response = await deletePostRecipe(data.post.id, recipeId);
        if (response.ok) {
            recipes = await getPostRecipes(data.post.id);
        }
    }

</script>

<div class="flex flex-col lg:flex-row">

    <div class="flex-1 form-control">
        <h3 class="font-bold text-lg py-5">Edit Post</h3>
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">Title</span>
        </label>
        <input type="text" min="1" bind:value={title} placeholder="Title" class="input input-bordered" />
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">Content</span>
        </label>
        <textarea class="textarea textarea-bordered" placeholder="Content" bind:value={content}></textarea>
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <!-- <label class="label">
            <span class="label-text">Media</span>
        </label>
        <ImageInput bind:files={files} multiple /> -->
        <button class="btn btn-primary w-fit mt-5" on:click={save}>Save</button>
    </div>

    <div class="flex-1">
        <h3 class="font-bold text-lg py-5">Attach Recipes</h3>
        <div class="flex gap-5 flex-col items-center">
            {#each recipes as recipe}
                <div class="flex indicator">
                    <button class="indicator-item badge badge-error text-lg" on:click={() => deleteRecipe(recipe.id)}>x</button>
                    <RecipeComponent {recipe} />
                </div>
            {/each}
        </div>
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
            <a class="btn btn-outline w-fit" href="/recipe/create">Create New Recipe</a>
        </label>
    </div>

</div>
