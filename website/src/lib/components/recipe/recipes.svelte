<script lang="ts">

    import RecipeComponent from '$lib/components/recipe/recipe.svelte';
    import { getContext } from 'svelte';
    import type { Recipe } from '$lib/app/recipe';
    import type { Writable } from 'svelte/store';

    const recipes: Writable<Recipe[]> = getContext('recipes');

</script>

<div class="flex gap-5 flex-col items-center">
    {#each $recipes as recipe}
        <RecipeComponent {recipe} />
    {/each}
</div>