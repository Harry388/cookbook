<script lang="ts">

    import RecipeComponent from '$lib/components/recipe/recipe.svelte';
    import type { Recipe } from '$lib/app/recipe';

    export let recipes: Recipe[];

</script>

{#each recipes as recipe}
    <RecipeComponent {recipe} link />
{/each}
