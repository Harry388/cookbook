<script lang="ts">

    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let value = '';
    export let title = '';
    export let edit = false;
    export let long = false;

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
<form on:submit|preventDefault={saveEdit} class="form-control">
    {#if long}
        <textarea id="input" bind:value={value} placeholder={title} class="input input-bordered" />
    {:else}
        <input id="input" type="text" min="1" bind:value={value} placeholder={title} class="input input-bordered" />
    {/if}
    {#if editing}
        <input type="submit" class="btn btn-success" value="Save" />
        <button class="btn btn-warning" on:click={cancelEdit}>Cancel</button>
    {/if}
</form>
