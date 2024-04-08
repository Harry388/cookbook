<script lang="ts">

    import Control from '$lib/components/cookbook/control.svelte';
    import { setContext } from 'svelte';
    import { writable } from 'svelte/store';

    export let page: number;
    const current_page = writable(page);
    $: $current_page = page;
    setContext('page', current_page);
    setContext('n', writable(0));
    setContext('pages', writable([]));

</script>

<Control />

<div class="flex flex-col print:flex-col lg:flex-row">
    <slot />
</div>
