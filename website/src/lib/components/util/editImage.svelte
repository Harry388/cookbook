<script lang="ts">

    import { createEventDispatcher } from 'svelte';
    import Image from '$lib/components/util/image.svelte';
    
    const dispatch = createEventDispatcher();

    export let src: string;

    let file: File;
    let input: HTMLInputElement;
    
    let reload = 0;

    function removeImage() {
        dispatch('remove');
    }

    function change() {
        if (input.files && input.files.length) {
            file = input.files[0];
            dispatch('change', { file, after: () => reload++ });
        }
    }

</script>

<div class="flex">
    <button on:click={removeImage} class="indicator-item badge badge-error text-base py-3"><i class="fa-solid fa-xmark"></i></button> 
    {#key reload}
        <Image {src} imageClass="rounded-lg shadow-lg" />
    {/key}
</div>

<label for={src} class="btn btn-outline">Select Image</label>
<input bind:this={input} on:change={change} id={src} type="file" accept="image/*" class="hidden">
