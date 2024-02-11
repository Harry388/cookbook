<script lang="ts">

    import { updateCommunity, deleteCommunity } from '$lib/app/community.js';
    import { invalidate, goto } from '$app/navigation';

    export let data;

    let title = data.community.title;
    let description = data.community.description;

    async function save() {
        const response = await updateCommunity(data.community.id, title, description).run();
        if (response.ok) {
            invalidate('app:community');
            history.back();
        }
    }

    async function remove() {
        if (!confirm('Are you sure?')) return;
        const response = await deleteCommunity(data.community.id).run();
        if (response.ok) {
            goto('/community').then(() => {
                invalidate('app:community');
            });
        }
    }

</script>

<button class="btn btn-error" on:click={remove}>Delete Community</button>

<h3 class="font-bold text-lg py-5">Edit Community</h3>
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
    <button class="btn btn-primary w-fit mt-5" on:click={save}>Save</button>
</div>
