<script lang="ts">

    import { createEventDispatcher } from 'svelte';
    import { onDestroy } from 'svelte';

    const dispatch = createEventDispatcher();

    export let files: File[] = [];
    export let multiple = false;

    let input: HTMLInputElement;

    $: urls = files.map(file => URL.createObjectURL(file));

    function onDelete(index: number) {
        URL.revokeObjectURL(urls[index]);
        files = files.filter((_, i) => index != i);
    }

    function fileType(file: File) {
        return file.type.split('/')[0];
    }

    function onChange() {
        if (input.files) {
            const newFiles = [...input.files].filter(f => ['image', 'video'].includes(fileType(f)));
            if (multiple) {
                files = [...files, ...newFiles];
            }
            else {
                files = [...newFiles];
            }
            input.files = null;
        }
        dispatch('change', files);
    }

    onDestroy(() => {
        for (const url of urls) {
            URL.revokeObjectURL(url);
        }
    })

</script>

<div class="flex flex-wrap gap-5 mb-5">
    {#each files as file, i}
        {@const url = urls[i]}
        <div class="self-start indicator w-fit lg:w-1/4">
            <button on:click={() => onDelete(i)} class="indicator-item badge badge-error text-base py-3"><i class="fa-solid fa-xmark"></i></button> 
            {#if fileType(file) == 'image'}
                <img src={url} alt="Post" class="rounded-lg shadow-lg">
            {:else if fileType(file) == 'video'}
                <video src={url} class="rounded-lg shadow-lg" controls autoplay loop>
                    <track kind="captions">
                    <source src={url}>
                </video>
            {/if}
        </div>
    {/each}
</div>

<label for="files" class="btn btn-outline">{ multiple ? 'Add Images' : 'Select Image' }</label>
<input bind:this={input} on:change={onChange} {multiple} id="files" type="file" accept="image/*,video/*" class="hidden">
