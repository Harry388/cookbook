<script lang="ts">

    import type { Page } from '$lib/app/page';

    export let pages: Page[];

</script>

<div class="pt-20 bg-base-200 m-auto w-fit">
    <h2 class="text-5xl font-bold">Contents</h2>
    {#each pages as page, i}
        <a href="?p={3 + i}" class="flex">
            {#if page.type == 'Section'}
                <h2 class="text-4xl font-bold my-5">{ page.position + 1}. { page.title }</h2>
            {:else}
                <h3 class="text-2xl">{ page.title }</h3>
            {/if}
            <div class="flex-grow mr-5"></div>
            <div class={page.type == 'Section' ? "text-4xl font-bold my-5" : "text-2xl"}>{ i + 3 }</div>
        </a>
    {/each}
    <div class="mt-5"></div>
    <a class="flex" href="?p={pages.length + 3}">
        <div class="text-4xl font-bold">Index</div>
        <div class="flex-grow mr-5"></div>
        <div class="text-4xl font-bold">{ pages.length + 3 }</div>
    </a>
</div>
