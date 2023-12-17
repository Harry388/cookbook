<script lang="ts">

    import { onDestroy } from 'svelte';

    export let files: File[] = [];
    export let multiple = false;

    let input: HTMLInputElement;

    $: images = files.map(file => URL.createObjectURL(file));

    function onDelete(index: number) {
        URL.revokeObjectURL(images[index]);
        files = files.filter((_, i) => index != i);
    }

    function onChange() {
        if (input.files) {
            if (multiple) {
                files = [...files, ...input.files];
            }
            else {
                files = [...input.files];
            }
            input.files = null;
        }
    }

    onDestroy(() => {
        for (const image of images) {
            URL.revokeObjectURL(image);
        }
    })

</script>

<div class="flex gap-5">
    {#each images as image, i}
        <div class="indicator w-1/5">
            <button on:click={() => onDelete(i)} class="indicator-item badge badge-secondary">x</button> 
            <img src={image} alt="Post Image">
        </div>
    {/each}
</div>

<label for="files" class="btn btn-success w-fit">{ multiple ? 'Add Images' : 'Select Image' }</label>
<input bind:this={input} on:change={onChange} {multiple} id="files" type="file" accept="image/*" class="hidden">