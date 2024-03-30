<script lang="ts">

    import type { Page } from '$lib/app/page';

    type IndexedPages = {
        [key: string]: (Page & { n: number })[]
    };

    export let pages: Page[];

    $: indexedPages = sortPages(pages);

    function sortPages(pages: Page[]) {
        const recipes = pages.map((p, i) => ({...p, n: i})).filter(p => p.type == 'Recipe');
        const indexedPages: IndexedPages = {};
        for (const page of recipes) {
            const start = page.title.charAt(0).toUpperCase();
            if (start in indexedPages) {
                indexedPages[start].push(page);
            }
            else {
                indexedPages[start] = [page];
            }
        }
        return indexedPages;
    }

</script>

<div class="pt-20 bg-base-200 m-auto w-fit">
    <h2 class="text-5xl font-bold">Index</h2>
    {#each Object.entries(indexedPages) as [start, pages]}
        <h2 class="text-4xl font-bold my-5">{ start }</h2>
        {#each pages as page}
            <a href="?p={3 + page.n}" class="flex">
                <h3 class="text-2xl">{ page.title }</h3>
                <div class="flex-grow mr-5"></div>
                <div class="text-2xl">{ page.n }</div>
            </a>
        {/each}
    {/each}
</div>
