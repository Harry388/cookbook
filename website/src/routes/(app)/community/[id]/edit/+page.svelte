<script lang="ts">

    import { updateCommunity, deleteCommunity } from '$lib/app/community.js';
    import { invalidate, goto } from '$app/navigation';
    import Input from '$lib/components/util/input.svelte';
    import Confirm from '$lib/components/util/confirm.svelte';

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
        const response = await deleteCommunity(data.community.id).run();
        if (response.ok) {
            goto('/community').then(() => {
                invalidate('app:community');
            });
        }
    }

</script>

<div class="lg:w-1/2 m-auto">
    <h3 class="font-bold text-lg py-5">Edit Community</h3>
    <div class="form-control">
        <Input bind:value={title} title="Title" edit on:save={save} />
        <Input bind:value={description} title="Description" edit on:save={save} long />
        <div class="form-control w-52">
            <label class="cursor-pointer label">
              <span class="label-text">Public Community</span> 
              <input type="checkbox" class="toggle" bind:checked={isPublic} on:change={save} />
            </label>
        </div>
    </div>

    <Confirm let:show on:confirm={remove} id={data.community.id}>
        <button class="btn btn-error w-full my-5" on:click={show}>Delete Community</button>
    </Confirm>
</div>
