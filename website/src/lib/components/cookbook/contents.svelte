<script lang="ts">

    import type { Page } from '$lib/app/page';

    export let pages: Page[];
    export let small = false;

    let header = small ? 'text-2xl' : 'text-5xl';
    let section = small ? 'text-xl' : 'text-4xl';
    let recipe = small ? 'text-md' : 'text-2xl';

</script>

<div class="{!small && 'pt-20'} m-auto w-fit">
    <a href="?p=2"><h2 class="{header} font-bold">Contents</h2></a>
    {#each pages as page, i}
        <a href="?p={3 + i}" class="flex">
            {#if page.type == 'Section'}
                <h2 class="{section} font-bold my-5">{ page.position + 1}. { page.title }</h2>
            {:else}
                <h3 class={recipe}>{ page.title }</h3>
            {/if}
            <div class="flex-grow mr-5"></div>
            <div class={page.type == 'Section' ? `${section} font-bold my-5` : recipe}>{ i + 3 }</div>
        </a>
    {/each}
    <div class="mt-5"></div>
    <a class="flex" href="?p={pages.length + 3}">
        <div class="{section} font-bold">Index</div>
        <div class="flex-grow mr-5"></div>
        <div class="{section} font-bold">{ pages.length + 3 }</div>
    </a>
</div>
