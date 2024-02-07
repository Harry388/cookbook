<script lang="ts">

    import { updateCommunity } from '$lib/app/community';

    export let data;

    let title = data.community.title;
    let description = data.community.description;

    async function save() {
        const response = await updateCommunity(data.id, title, description);
        if (response.ok) {
            history.back();
        }
    }

</script>

<h3 class="font-bold text-lg py-5">Edit Profile</h3>
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
    <button class="btn btn-primary w-fit mt-5" on:click={save}>Save</button>
</div>
