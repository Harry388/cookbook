<script lang="ts">

    import { getContext } from 'svelte';
    import { goto } from '$app/navigation';
    import Contents from '$lib/components/cookbook/contents.svelte';
    import type { Writable } from 'svelte/store';

    const currentPage: Writable<number> = getContext('page');
    const pages: Writable<any[]> = getContext('pages');

    $: {
        if ($currentPage < 0) {
            goto('?p=0');
        }
        else if ($currentPage >= $pages.length) {
            goto(`?p=${$pages.length - 1}`);
        }
    }

</script>

<div class="flex print:hidden w-full p-2">
    <div class="flex-1">
        {#if $currentPage > 0}
            <a href="?p={$currentPage - 1}" class="btn btn-circle flex-1">❮</a> 
        {/if}
    </div>
    <div class="dropdown">
        <div tabindex="0" role="button" class="fa-solid fa-list-ul text-2xl mt-2"></div>
        <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
        <div tabindex="0" class="dropdown-content z-[1] menu p-5 shadow bg-base-100 rounded-box w-52">
            <Contents small />
        </div>
    </div>
    <div class="flex-1 flex justify-end">
        {#if $currentPage < ($pages.length - 1)}
            <a href="?p={$currentPage + 1}" class="btn btn-circle">❯</a>
        {/if}
    </div>
</div>
