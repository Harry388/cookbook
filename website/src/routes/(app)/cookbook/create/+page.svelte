<script lang="ts">

    import { createCookbook } from '$lib/app/cookbook';
    import Input from '$lib/components/util/input.svelte';

    let title = '';
    let description = '';

    async function create() {
        const response = await createCookbook(title, description).run();
        if (response.ok) {
            history.back();
        }
    }

</script>

<div class="lg:w-1/2 m-auto">
    <h3 class="font-bold text-lg py-5">Create a Cookbook</h3>
    <div class="form-control">
        <Input bind:value={title} title="Title" />
        <Input bind:value={description} title="Description" long />
        <button class="btn btn-primary w-fit mt-5" on:click={create}>Create</button>
    </div>
</div>
