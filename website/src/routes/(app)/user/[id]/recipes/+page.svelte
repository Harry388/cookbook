<script lang="ts">

    import Recipes from '$lib/components/recipe/recipes.svelte';
    import { setContext } from 'svelte';
    import { writable } from 'svelte/store';
    import type { Recipe } from '$lib/app/recipe';

    export let data;

    const recipes = writable<Recipe[]>();
    $: recipes.set(data.recipes);
    setContext('recipes', recipes);

</script>


{#if data.self}
    <a class="btn btn-outline" href="/recipe/create">Create Recipe</a>
{/if}

<Recipes />