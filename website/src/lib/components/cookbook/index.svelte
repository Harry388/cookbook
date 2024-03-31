<script lang="ts">


    import { getContext } from 'svelte';
    import type { Writable } from 'svelte/store';

    type IndexedPages = {
        [key: string]: ({ title: string, header: boolean, n: number })[]
    };

    const pages: Writable<{ title: string, header: boolean }[]> = getContext('pages');

    $: indexedPages = sortPages($pages);

    function sortPages(pages: { title: string, header: boolean }[]) {
        const recipes = pages.map((p, i) => ({...p, n: i})).filter(p => p.title && !p.header);
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
            <a href="?p={page.n}" class="flex">
                <h3 class="text-2xl">{ page.title }</h3>
                <div class="flex-grow mr-5"></div>
                <div class="text-2xl">{ page.n }</div>
            </a>
        {/each}
    {/each}
</div>
