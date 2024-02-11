<script lang="ts">

    import Recipes from '$lib/components/recipe/recipes.svelte';

    export let data;

</script>


{#if data.self}
    <a class="btn btn-outline" href="/recipe/create">Create Recipe</a>
{/if}

<Recipes recipes={data.recipes} />
