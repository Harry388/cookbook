<script context="module" lang="ts">

    export type Option = {
        id: number,
        title: string
    };

</script>

<script lang="ts">
    
    export let title = '';
    export let options: Option[];
    export let value: number;

</script>

{#if title}
    <label class="label" for="#select">
        <span class="label-text">{ title }</span>
    </label>
{/if}
<select id="select" bind:value={value} class="select select-bordered w-full">
    <option value={-1} selected>{ title }</option>
    {#each options as option}
        <option value={option.id}>{ option.title }</option>
    {/each}
</select>
