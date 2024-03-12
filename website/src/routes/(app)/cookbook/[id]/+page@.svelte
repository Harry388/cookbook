<script lang="ts">

    import Recipe from '$lib/components/cookbook/recipe.svelte';

    export let data;

    $: allowed = data.page % 2 ? [data.page - 1, data.page] : [data.page, data.page + 1];

</script>

<svelte:head>
    <title>{ data.cookbook.title } - Cookbook</title>
</svelte:head>

<div class="flex">
    {#if data.id == data.cookbook.user_id}
        <a href="/cookbook/{data.cookbook.id}/edit" class="btn btn-outline">Edit</a>
    {/if}
    {#if data.page > 0}
        <a href="?p={data.page - 2}" class="btn btn-circle">❮</a> 
    {/if}
    <div class="flex-grow"></div>
    {#if data.page < (data.recipes.length - 1)}
        <a href="?p={data.page + 2}" class="btn btn-circle">❯</a>
    {/if}
</div>

<div class="flex">
    {#each data.recipes as recipe, i}
        {#if allowed.includes(i)}
            <div class="w-1/2">
                <Recipe {recipe} />
            </div>
        {/if}
    {/each}
</div>

