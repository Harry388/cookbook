<script lang="ts">

    import { getContext } from 'svelte';
    import type { Writable } from 'svelte/store';

    export let hideNumber = false;

    const currentPage: Writable<number> = getContext('page');
    const n: Writable<number> = getContext('n');
    const thisPage = $n;
    $n += 1;

</script>

<div class="{($currentPage != thisPage) && 'hidden'} print:block w-full print:h-screen">
    {#if !hideNumber}
        <div class="print:hidden w-fit m-auto mb-5">
            { thisPage }
        </div>
    {/if}
    <slot />
</div>
