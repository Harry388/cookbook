<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';
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

<div class="flex flex-col items-center lg:items-start lg:flex-row  mt-5 justify-center gap-x-72 gap-y-5">

    <Recipe recipe={data.recipe} />

    <div>

        <Posts posts={data.posts} />

    </div>

</div>

