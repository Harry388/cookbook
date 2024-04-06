<script lang="ts">

    import Recipes from '$lib/components/recipe/recipes.svelte';

    export let data;

</script>

<div class="lg:w-1/3 lg:m-auto flex flex-col">
    {#if data.self}
        <a href="/recipe/create" class="btn btn-outline mb-5"><i class="fa-solid fa-plus"></i>Create Recipe</a>
    {/if}
    {#if data.recipes.length == 0}
        <p class="text-lg text-center">No recipes found</p>
    {/if}
    <Recipes recipes={data.recipes} />
</div>
