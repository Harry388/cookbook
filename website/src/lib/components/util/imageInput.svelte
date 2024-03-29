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

<div class="flex flex-wrap gap-5 mb-5">
    {#each images as image, i}
        <div class="self-start indicator w-fit lg:w-1/4">
            <button on:click={() => onDelete(i)} class="indicator-item badge badge-error text-base py-3"><i class="fa-solid fa-xmark"></i></button> 
            <img src={image} alt="Post" class="rounded-lg shadow-lg">
        </div>
    {/each}
</div>

<label for="files" class="btn btn-success w-fit">{ multiple ? 'Add Images' : 'Select Image' }</label>
<input bind:this={input} on:change={onChange} {multiple} id="files" type="file" accept="image/*" class="hidden">
