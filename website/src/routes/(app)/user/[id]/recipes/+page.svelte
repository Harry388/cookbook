<script lang="ts">

    import Recipes from '$lib/components/recipe/recipes.svelte';

    export let data;

</script>


{#if data.self}
    <a class="btn btn-outline" href="/recipe/create">Create Recipe</a>
{/if}

<div class="w-11/12 lg:w-1/3 m-auto">
    <Recipes recipes={data.recipes} />
</div>
