<script lang="ts">

    import Recipes from '$lib/components/recipe/recipes.svelte';

    export let data;

</script>

{#if data.recipes.length == 0}
    <p class="text-lg text-center">No recipes found</p>
{/if}
<Recipes recipes={data.recipes} />
