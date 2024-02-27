<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';
    import CommentBlock from '$lib/components/comment/commentBlock.svelte';
    import Tags from '$lib/components/tag/tags.svelte';
    import { deleteRecipe } from '$lib/app/recipe';
    import { goto } from '$app/navigation';

    export let data;


    async function onDelete() {
        if (!confirm('Are you sure?')) return;
        const response = await deleteRecipe(data.recipe.id).run();
        if (response.ok) {
            goto(`/user/${data.id}/recipes`);
        }
    }

</script>

{#if data.ownsRecipe}
    <a class="btn btn-primary" href="/recipe/{data.recipe.id}/edit">Edit Recipe</a>
    <button class="btn btn-error" on:click={onDelete}>Delete Recipe</button>
{/if}

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
