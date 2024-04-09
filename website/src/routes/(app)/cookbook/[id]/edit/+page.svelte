<script lang="ts">

    import EditSection from '$lib/components/cookbook/editSection.svelte';
    import Input from '$lib/components/util/input.svelte';
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

<div class="lg:w-1/2 m-auto">

    <div class="form-control">
        <h3 class="font-bold text-lg py-5">Edit Cookbook</h3>
        <Input bind:value={title} title="Title" edit on:save={save} />
        <Input bind:value={description} title="Description" edit on:save={save} />
    </div>

    <div class="flex flex-col gap-y-5">
        <h3 class="font-bold text-lg py-5">Edit Pages</h3>
        {#each book as section}
            <EditSection {section} cookbookId={data.cookbook.id} on:change={resetPages} />
        {/each}
        <div class="form-control">
            <Input bind:value={newSection} title="New Section" />
            <button class="btn btn-primary w-full btn-outline mt-5" on:click={addSection}>Add</button>
        </div>
    </div>
</div>
