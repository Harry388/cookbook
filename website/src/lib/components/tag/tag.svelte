<script lang="ts">

    import { createEventDispatcher } from 'svelte';
    import type { Tag } from '$lib/app/tag';

    export let tag: Tag
    export let edit = false;

    const dispatch = createEventDispatcher();

    function remove() {
        dispatch('remove', tag);
    }

</script>

<svelte:element this={edit ? 'div' : 'a'} href="/tag/{tag.id}" class="badge badge-neutral gap-2">
    {#if edit}
        <button on:click={remove}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-4 h-4 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
    {/if}
    { tag.tag }
</svelte:element>
