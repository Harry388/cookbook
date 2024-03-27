<script lang="ts">

    import { getContext } from 'svelte';
    import type { Writable } from 'svelte/store';

    const current_page: Writable<number> = getContext('page');
    const n: Writable<number> = getContext('n');
    const this_page = $n;
    $n += 1;

</script>

<div class="{($current_page != this_page) && 'hidden'} print:block w-full print:h-screen">
    <slot />
</div>
