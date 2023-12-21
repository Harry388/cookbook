<script lang="ts">

    import { updatePost } from '$lib/app/post';

    export let data;

    let title = data.post.title;
    let content = data.post.content;
    let recipe: number;

    $: console.log(recipe);

    async function save() {
        const response = await updatePost(data.post.id, title, content);
        if (response.ok) {
            history.back();
        }
    }

</script>

<div class="flex">

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
        <label class="form-control w-full max-w-xs">
            <div class="label">
                <span class="label-text">Pick Recipe</span>
            </div>
            <select bind:value={recipe} class="select select-bordered">
                <option disabled selected>Pick one</option>
                {#each data.userRecipes as recipe}
                    <option value={recipe.id}>{ recipe.title }</option>
                {/each}
            </select>
        </label>
    </div>

</div>
