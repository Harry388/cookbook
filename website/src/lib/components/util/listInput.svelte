<script lang="ts">

    import Input from '$lib/components/util/input.svelte';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let list: string[] = [];
    export let edit = false;
    export let title = '';
    export let placeholder = title;

    let internalList = [...list, ''];
    let rerender = 0;

    $: list = internalList.filter(i => i != '');

    $: {
        const last = internalList[internalList.length - 1];
        if (last != '') {
            internalList = [...internalList, ''];
        }
    }

    function remove(index: number) {
        if (internalList.length > 1) {
            internalList = internalList.filter((_, i) => index != i);
            dispatch('change');
            rerender++;
        }
    }

</script>

{#if title}
    <label for="#input" class="label">
        <span class="label-text">{ title }</span>
    </label>
{/if}
{#key rerender}
    {#each internalList as item, i}
        <div id="input" class="flex items-center mb-3">
            <p class="mr-5">{i + 1}.</p>
            <Input bind:value={item} {edit} {placeholder} on:save={() => dispatch('change')} />
            {#if i != (internalList.length - 1)}
                <button class="fa-regular fa-trash-can text-2xl ml-5" on:click={() => remove(i)}></button>
            {/if}
        </div>
    {/each}
{/key}
