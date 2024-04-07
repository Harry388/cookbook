<script lang="ts">

    import Input from '$lib/components/util/input.svelte';
    import { setContext } from 'svelte';
    import { writable } from 'svelte/store';

    const search = writable('');

    setContext('search', search);

</script>

<Input bind:value={$search} placeholder="Search" />

<slot />
