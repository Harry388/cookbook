<script lang="ts">

    import { createCommunity } from '$lib/app/community';
    import { goto } from '$app/navigation';

    let title = '';
    let description = '';

    async function create() {
        const response = await createCommunity(title, description);
        if (response.ok) {
            goto('/community');
        }
    }

</script>

<h3 class="font-bold text-lg py-5">Create Community</h3>
<div class="form-control">
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Title</span>
    </label>
    <input type="text" min="1" bind:value={title} placeholder="Title" class="input input-bordered" />
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Description</span>
    </label>
    <textarea class="textarea textarea-bordered" placeholder="Desciption" bind:value={description}></textarea>
    <button class="btn btn-primary w-fit mt-5" on:click={create}>Create</button>
</div>
