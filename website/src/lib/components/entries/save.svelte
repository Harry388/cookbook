<script lang="ts">

    import { getContext } from 'svelte';
    import { getUserAblums, addAlbumEntry } from '$lib/app/album';
    import { onMount } from 'svelte';
    import type { Album } from '$lib/app/album';

    const id: number = getContext('id');

    export let entryId: number;
    export let type: 'recipe' | 'post';

    let albums: Album[] = [];
    let selectAlbum: number;

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

    onMount(async () => {
        albums = await getUserAblums(id).json();
    });

</script>

<button class="fa-regular fa-bookmark text-2xl" on:click={show}></button>
<dialog id="modal" class="modal modal-bottom sm:modal-middle">
    <div class="modal-box">
        <h3 class="font-bold text-lg">Save to Album</h3>
        <select bind:value={selectAlbum} class="my-10 w-full select select-bordered">
            <option value={-1} selected>Pick Album</option>
            {#each albums as album}
                <option value={album.id}>{ album.title }</option>
            {/each}
        </select>
        <div class="modal-action">
            <form method="dialog">
                <button class="btn btn-ghost mr-5" on:click={cancel}>Cancel</button>
                <button class="btn" on:click={addToAlbum}>Save</button>
            </form>
        </div>
    </div>
    <form method="dialog" class="modal-backdrop">
        <button on:click={cancel}>close</button>
    </form>
</dialog>
