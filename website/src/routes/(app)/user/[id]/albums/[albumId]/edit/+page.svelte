<script lang="ts">

    import { updateAlbum, deleteAlbum } from '$lib/app/album';
    import { invalidate, goto } from '$app/navigation';

    export let data;

    let title = data.album.title;
    
    async function save() {
        const response = await updateAlbum(data.album.id, title).run();
        if (response.ok) {
            await Promise.all([invalidate('app:album'), invalidate('app:albums')]);
            goto(`/user/${data.user.id}/albums/${data.album.id}`); 
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

<button class="btn btn-error" on:click={remove}>Delete Album</button>

<h3 class="font-bold text-lg py-5">Edit Album</h3>
<div class="form-control">
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Title</span>
    </label>
    <input type="text" min="1" bind:value={title} placeholder="Title" class="input input-bordered" />
    <button class="btn btn-primary w-fit mt-5" on:click={save}>Save</button>
</div>
