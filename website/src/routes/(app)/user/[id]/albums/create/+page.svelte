<script lang="ts">

    import { createAlbum } from '$lib/app/album';
    import { invalidate } from '$app/navigation';
    import Input from '$lib/components/util/input.svelte';

    let title = '';

    async function create() {
        if (!title) return;
        const response = await createAlbum(title).run();
        if (response.ok) {
            await invalidate('app:albums');
            history.back();
        }
    }

</script>

<button on:click={() => history.back()} class="text-lg fa-solid fa-arrow-left-long"></button>

<div class="lg:w-1/2 m-auto">
    <h3 class="font-bold text-lg py-5">Create Album</h3>
    <div class="form-control">
        <Input bind:value={title} title="Title" required />
        <button class="btn btn-success btn-outline mt-5" on:click={create}>Create</button>
    </div>
</div>
