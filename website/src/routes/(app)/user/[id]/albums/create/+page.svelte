<script lang="ts">

    import { createAlbum } from '$lib/app/album';
    import { invalidate } from '$app/navigation';

    let title = '';
    
    async function create() {
        const response = await createAlbum(title).run();
        if (response.ok) {
            await invalidate('app:albums');
            history.back();
        }
    }

</script>

<h3 class="font-bold text-lg py-5">Create Album</h3>
<div class="form-control">
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Title</span>
    </label>
    <input type="text" min="1" bind:value={title} placeholder="Title" class="input input-bordered" />
    <button class="btn btn-primary w-fit mt-5" on:click={create}>Create</button>
</div>
