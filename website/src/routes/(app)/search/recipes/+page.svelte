<script lang="ts">

    import Recipes from '$lib/components/recipe/recipes.svelte';

    export let data;

</script>

<Recipes recipes={data.recipes} />
