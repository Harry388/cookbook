<script lang="ts">

    import { updateAlbum, deleteAlbum } from '$lib/app/album';
    import { invalidate, goto } from '$app/navigation';
    import Input from '$lib/components/util/input.svelte';

    export let data;

    let title = data.album.title;
    
    async function save() {
        const response = await updateAlbum(data.album.id, title).run();
        if (response.ok) {
            await Promise.all([invalidate('app:album'), invalidate('app:albums')]);
        }
    }

    async function remove() {
        if (!confirm('Are you sure?')) return;
        const response = await deleteAlbum(data.album.id).run();
        if (response.ok) {
            await invalidate('app:albums');
            goto(`/user/${data.user.id}/albums`); 
        }
    }

</script>

<a href="/user/{data.user.id}/albums/{data.album.id}"><i class="text-lg fa-solid fa-arrow-left-long"></i></a>

<h3 class="font-bold text-lg py-5">Edit Album</h3>
<div class="form-control">
    <Input bind:value={title} title="Title" edit on:save={save} />
</div>

<button class="btn btn-error my-5" on:click={remove}>Delete Album</button>
