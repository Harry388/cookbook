<script lang="ts">

    import Tags from '$lib/components/tag/tags.svelte';
    import { getEntryTags } from '$lib/app/tag';
    import { onMount } from 'svelte';
    import type { Recipe } from '$lib/app/recipe';
    import type { Tag } from '$lib/app/tag';

    export let recipe: Recipe;

    let tags: Tag[] = [];

    onMount(async () => {
        tags = await getEntryTags(recipe.id, 'recipe').json();
    });

</script>

<div class="flex flex-col w-fit m-auto gap-3">
    <h1 class="font-bold text-4xl">{ recipe.title }</h1>
    <Tags {tags} />
    <p class="text-lg">{ recipe.description }</p>
    <h2 class="font-bold text-3xl">Ingredients</h2>
    {#each recipe.ingredients as ingredient, i}
        <div class="text-lg">{ i + 1}. { ingredient }</div>
    {/each}
    <h2 class="font-bold text-3xl">Method</h2>
    {#each recipe.method as step, i}
        <div class="text-lg">{ i + 1}. { step }</div>
    {/each}
</div>
