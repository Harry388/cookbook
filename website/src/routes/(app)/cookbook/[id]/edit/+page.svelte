<script lang="ts">

    import EditSection from '$lib/components/cookbook/editSection.svelte';
    import { formatPageArray } from '$lib/app/page';
    import { invalidate } from '$app/navigation';
    import { updateCookbook, addCookbookSection, getCookbookPages } from '$lib/app/cookbook';

    export let data;

    let title = data.cookbook.title;
    let description = data.cookbook.description;
    let book = formatPageArray(data.pages);
    let newSection = '';

    async function resetPages() {
        const pages = await getCookbookPages(data.cookbook.id).json();
        book = formatPageArray(pages);
    }

    async function save() {
        const response = await updateCookbook(data.cookbook.id, title, description).run();
        if (response.ok) {
            invalidate('app:cookbook');
            history.back();
        }
    }

    async function addSection() {
        const response = await addCookbookSection(data.cookbook.id, newSection).run();
        if (response.ok) {
            newSection = '';
            await resetPages();
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
        {#each book as section}
            <EditSection {section} cookbookId={data.cookbook.id} on:change={resetPages} userRecipes={data.userRecipes} />
        {/each}
        <div class="form-control">
            <!-- svelte-ignore a11y-label-has-associated-control -->
            <label class="label">
                <span class="label-text">New Section</span>
            </label>
            <input type="text" min="1" bind:value={newSection} placeholder="New Section" class="input input-bordered" />
            <button class="btn btn-primary w-fit mt-5" on:click={addSection}>Add</button>
        </div>
    </div>
</div>
