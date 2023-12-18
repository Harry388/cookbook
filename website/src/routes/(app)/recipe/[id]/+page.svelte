<script lang="ts">

    import Recipe from '$lib/components/recipe/recipe.svelte';
    import { deleteRecipe } from '$lib/app/recipe';
    import { goto } from '$app/navigation';

    export let data;

    async function onDelete() {
        if (!confirm('Are you sure?')) return;
        const response = await deleteRecipe(data.recipe.id);
        if (response.ok) {
            goto(`/user/${data.id}/recipes`);
        }
    }

</script>

<a class="btn btn-outline" href="/user/{data.recipe.user_id}/recipes">Back</a>

{#if data.ownsRecipe}
    <a class="btn btn-primary" href="/recipe/{data.recipe.id}/edit">Edit Recipe</a>
    <button class="btn btn-error" on:click={onDelete}>Delete Recipe</button>
{/if}

<div class="flex flex-col items-center lg:flex-row  mt-5">

    <Recipe recipe={data.recipe} />

</div>

