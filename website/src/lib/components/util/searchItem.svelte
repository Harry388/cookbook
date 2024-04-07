<script lang="ts">

    import { getContext } from 'svelte';
    import type { Writable } from 'svelte/store';

    const search: Writable<string> = getContext('search');

    export let key: string;

    $: show =  ($search.length == 0) || (key.toLowerCase().includes($search.toLowerCase()));

</script>

<div class={show ? '' : 'hidden'}>
    <slot />
</div>
