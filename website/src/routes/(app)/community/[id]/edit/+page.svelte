<script lang="ts">

    import { updateCommunity, deleteCommunity } from '$lib/app/community.js';
    import { invalidate, goto } from '$app/navigation';
    import Input from '$lib/components/util/input.svelte';

    export let data;

    let title = data.community.title;
    let description = data.community.description || '';
    let isPublic = Boolean(data.community.public);

    async function save() {
        const response = await updateCommunity(data.community.id, title, description, isPublic).run();
        if (response.ok) {
            invalidate('app:community');
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

<h3 class="font-bold text-lg py-5">Edit Community</h3>
<div class="form-control">
    <Input bind:value={title} title="Title" edit on:save={save} />
    <Input bind:value={description} title="Description" edit on:save={save} long />
    <label class="label" for="#public">
        <span class="label-text">Public</span>
    </label>
    <input id="#public" type="checkbox" class="checkbox checkbox-primary" bind:checked={isPublic} on:change={save} />
</div>

<button class="btn btn-error my-5" on:click={remove}>Delete Community</button>
