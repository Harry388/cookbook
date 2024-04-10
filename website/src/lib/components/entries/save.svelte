<script lang="ts">

    import Input from '$lib/components/util/input.svelte';
    import SelectInput from '$lib/components/util/selectInput.svelte';
    import { getContext } from 'svelte';
    import { getUserAblums, addAlbumEntry, createAlbum } from '$lib/app/album';
    import { onMount } from 'svelte';
    import type { Album } from '$lib/app/album';

    const id: number = getContext('id');

    export let entryId: number;
    export let type: 'recipe' | 'post';

    let albums: Album[] = [];
    let selectAlbum: number;
    let create = false;
    let createTitle = '';

    let close: HTMLButtonElement;

    function show() {
        //@ts-ignore
        document.getElementById('modal').showModal();
    }

    function cancel() {
        selectAlbum = -1;
    }

    async function addToAlbum() {
        if (selectAlbum == -1) return;
        const response = await addAlbumEntry(selectAlbum, entryId, type).run();
        if (response.ok) {
            selectAlbum = -1;
        }
    }

    async function saveAlbum() {
        const response = await createAlbum(createTitle).run();
        if (response.ok) {
            selectAlbum = await response.json(); 
            await addToAlbum();
            close.click();
        }
    }

    onMount(async () => {
        albums = await getUserAblums(id).json();
    });

</script>

<button class="fa-regular fa-bookmark text-2xl" on:click={show}></button>
<dialog id="modal" class="modal modal-bottom sm:modal-middle">
    <div class="modal-box">
        <h3 class="font-bold text-lg">Save to Album</h3>
        <SelectInput bind:value={selectAlbum} options={albums} title="Pick Album" />
        <div class="divider divider-vertical">OR</div>
        {#if !create}
            <button class="btn btn-outline w-full" on:click={() => create = true}>Create Album</button>
        {:else}
            <Input bind:value={createTitle} title="Title" />
            <button class="btn btn-outline btn-success w-full mt-5" on:click={saveAlbum}>Save</button>
        {/if}
        <div class="modal-action">
            <form method="dialog">
                <button class="btn btn-ghost mr-5" on:click={cancel}>Cancel</button>
                <button class="btn" on:click={addToAlbum}>Save</button>
            </form>
        </div>
    </div>
    <form method="dialog" class="modal-backdrop">
        <button on:click={cancel} bind:this={close}>close</button>
    </form>
</dialog>
