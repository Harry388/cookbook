<script lang="ts">

    import RecipeComponent from '$lib/components/recipe/recipe.svelte';
    import type { Recipe } from '$lib/app/recipe';

    export let recipes: Recipe[];

</script>

{#each recipes as recipe (recipe.id)}
    <div class="mt-5"></div>
    <RecipeComponent {recipe} link />
{/each}
