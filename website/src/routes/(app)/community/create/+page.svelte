<script lang="ts">

    import { createCommunity } from '$lib/app/community';
    import { invalidate, goto } from '$app/navigation';
    import Input from '$lib/components/util/input.svelte';

    let title = '';
    let description = '';

    async function create() {
        const response = await createCommunity(title, description).run();
        if (response.ok) {
            await invalidate('app:community');
            await goto('/community');
        }
    }

</script>

<div class="lg:w-1/2 m-auto">
    <h3 class="font-bold text-lg py-5">Create Community</h3>
    <div class="form-control">
        <Input bind:value={title} title="Title" required />
        <Input bind:value={description} title="Description" long />
        <button class="btn btn-success btn-outline mt-5" on:click={create}>Create</button>
    </div>
</div>
