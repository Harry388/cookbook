<script lang="ts">

    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let value = '';
    export let title = '';
    export let edit = false;
    export let long = false;
    export let placeholder = title;

    let oldValue = value;

    $: editing = edit && (oldValue != value);

    function saveEdit() {
        if (editing) {
            oldValue = value;
            dispatch('save', value);
        }
    }

    function cancelEdit() {
        value = oldValue;
    }

</script>

{#if title}
    <label for="#input" class="label">
        <span class="label-text">{ title }</span>
    </label>
{/if}
<form on:submit|preventDefault={saveEdit} class="form-control flex-row gap-x-5 items-center">
    {#if long}
        <textarea id="input" bind:value={value} {placeholder} class="flex-1 input input-bordered" />
    {:else}
        <input id="input" type="text" min="1" bind:value={value} {placeholder} class="flex-1 input input-bordered" />
    {/if}
    {#if editing}
        <div class="flex gap-x-5">
            <button on:click={saveEdit} class="fa-regular fa-floppy-disk text-2xl"></button>
            <button class="fa-solid fa-xmark text-2xl" on:click={cancelEdit}></button>
        </div>
    {/if}
</form>
