<script lang="ts">

    import Input from '$lib/components/util/input.svelte';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let list: string[] = [''];
    export let edit = false;
    export let title = '';

    let rerender = 0;

    function add() {
        list = [...list, ''];
        dispatch('change');
        rerender++;
    }

    function remove(index: number) {
        list = list.filter((_, i) => index != i);
        dispatch('change');
        rerender++;
    }

</script>

{#if title}
    <label for="#input" class="label">
        <span class="label-text">{ title }</span>
    </label>
{/if}
{#key rerender}
    {#each list as item, i}
        <div id="input" class="flex items-center">
            <p class="mr-5">{i + 1}.</p>
            <Input bind:value={item} {edit} on:save={() => dispatch('change')} />
            <button class="btn btn-outline w-fit" on:click={() => remove(i)}>Remove</button>
        </div>
    {/each}
{/key}

<button class="btn btn-outline w-fit" on:click={add}>Add</button>
