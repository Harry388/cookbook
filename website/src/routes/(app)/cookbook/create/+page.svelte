<script lang="ts">

    import { createCookbook } from '$lib/app/cookbook';

    let title = '';
    let description = '';

    async function create() {
        const response = await createCookbook(title, description).run();
        if (response.ok) {
            history.back();
        }
    }

</script>

<h3 class="font-bold text-lg py-5">Create a Cook Book</h3>
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
    <textarea class="textarea textarea-bordered" placeholder="Description" bind:value={description}></textarea>
    <button class="btn btn-primary w-fit mt-5" on:click={create}>Create</button>
</div>
