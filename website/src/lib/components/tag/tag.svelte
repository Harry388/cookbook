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

<svelte:element this={edit ? 'div' : 'a'} href="/tag/{tag.id}" class="badge badge-neutral gap-2 text-base p-4">
    {#if edit}
        <button class="fa-solid fa-xmark" on:click={remove}></button>
    {/if}
    { tag.tag }
</svelte:element>
