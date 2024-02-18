<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';
    import CommentBlock from '$lib/components/comment/commentBlock.svelte';
    import Tags from '$lib/components/tag/tags.svelte';
    import { deleteRecipe } from '$lib/app/recipe';
    import { addAlbumEntry } from '$lib/app/album';
    import { goto } from '$app/navigation';

    export let data;

    let newAlbum: number;

    async function onDelete() {
        if (!confirm('Are you sure?')) return;
        const response = await deleteRecipe(data.recipe.id).run();
        if (response.ok) {
            goto(`/user/${data.id}/recipes`);
        }
    }

    function addAlbum() {
       addAlbumEntry(newAlbum, data.recipe.id, 'recipe').run();
    }

</script>

{#if data.ownsRecipe}
    <a class="btn btn-primary" href="/recipe/{data.recipe.id}/edit">Edit Recipe</a>
    <button class="btn btn-error" on:click={onDelete}>Delete Recipe</button>
{/if}

<label class="form-control w-full max-w-xs">
    <div class="label">
        <span class="label-text">Pick Album</span>
    </div>
    <select bind:value={newAlbum} class="select select-bordered">
        <option value={-1} selected>Pick one</option>
        {#each data.albums as album}
            <option value={album.id}>{ album.title }</option>
        {/each}
    </select>
    <button class="btn btn-primary w-fit my-5" on:click={addAlbum}>Save to Album</button>
</label>

<Tags tags={data.tags} />

<div class="flex flex-col items-center lg:items-start lg:flex-row  mt-5 justify-center gap-4">

    <div class="basis-1/3">
        <Recipe recipe={data.recipe} />
    </div>

    <div class="basis-1/3">
        <CommentBlock comments={data.comments} type="RECIPE" id={data.recipe.id} />
    </div>
    
    <div class="basis-1/3">
        <Posts posts={data.posts} />
    </div>

</div>
