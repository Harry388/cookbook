<script lang="ts">


    import { getContext } from 'svelte';
    import type { Writable } from 'svelte/store';

    const pages: Writable<{ title: string, header: boolean }[]> = getContext('pages');

    export let small = false;

    let header = small ? 'text-2xl' : 'text-5xl';
    let section = small ? 'text-xl' : 'text-4xl';
    let recipe = small ? 'text-md' : 'text-2xl';

</script>

<div class="{!small && 'pt-20'} m-auto w-fit">
    <a href="?p=2"><h2 class="{header} font-bold">Contents</h2></a>
    {#each $pages as page, i}
        {#if page.title}
            <a href="?p={i}" class="flex">
                {#if page.header }
                    <h2 class="{section} font-bold my-5">{ page.title }</h2>
                {:else}
                    <h3 class={recipe}>{ page.title }</h3>
                {/if}
                <div class="flex-grow mr-5"></div>
                <div class={page.header ? `${section} font-bold my-5` : recipe}>{ i }</div>
            </a>
        {/if}
    {/each}
    <div class="mt-5"></div>
    <a class="flex" href="?p={$pages.length - 1}">
        <div class="{section} font-bold">Index</div>
        <div class="flex-grow mr-5"></div>
        <div class="{section} font-bold">{ $pages.length - 1 }</div>
    </a>
</div>
