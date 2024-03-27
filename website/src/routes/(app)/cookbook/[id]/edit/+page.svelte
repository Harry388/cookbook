<script lang="ts">

    import Recipe from '$lib/components/recipe/recipe.svelte';
    import { invalidate } from '$app/navigation';
    import { updateCookbook } from '$lib/app/cookbook';

    export let data;

    let title = data.cookbook.title;
    let description = data.cookbook.description;

    async function save() {
        const response = await updateCookbook(data.cookbook.id, title, description).run();
        if (response.ok) {
            invalidate('app:cookbook');
            history.back();
        }
    }

</script>

<div class="flex flex-col">

    <div class="form-control">
        <h3 class="font-bold text-lg py-5">Edit Cookbook</h3>
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">Title</span>
        </label>
        <input type="text" min="1" bind:value={title} placeholder="Title" class="input input-bordered" />
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">Description</span>
        </label>
        <textarea class="textarea textarea-bordered" placeholder="Description" bind:value={description}></textarea>
        <button class="btn btn-primary w-fit mt-5" on:click={save}>Save</button>
    </div>

    <div class="w-1/3 flex flex-col gap-y-5">
        <h3 class="font-bold text-lg py-5">Edit Pages</h3>
        {#each data.pages as page}
            {#if page.type == 'Recipe'}
                <Recipe recipe={page} link />
            {:else if page.type == 'Section'}
                <h2 class="font-bold text-2xl">{ page.title }</h2>
            {/if}
        {/each}
    </div>
</div>
