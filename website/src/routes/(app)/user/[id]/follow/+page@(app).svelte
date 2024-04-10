<script lang="ts">

    import MemberList from '$lib/components/user/memberList.svelte';
    import { acceptFollow, removeFollower } from '$lib/app/follow';
    import { invalidate } from '$app/navigation';

    export let data;

    async function remove(followerId: number) {
        const response = await removeFollower(followerId, data.id).run();
        if (response.ok) {
            invalidate('app:userFollow');
        }
    }

    async function unfollow(followingId: number) {
        const response = await removeFollower(data.id, followingId).run();
        if (response.ok) {
            invalidate('app:userFollow');
        }
    }

    async function accept(userId: number) {
        const response = await acceptFollow(userId).run();
        if (response.ok) {
            invalidate('app:userFollow');
        }
    }

</script>

<div class="flex">

    <MemberList members={data.followers} let:id>
        {#if data.self}
            <button class="btn btn-ghost" on:click={() => remove(id)}>Remove</button>
        {/if}
        <svelte:fragment slot="fallback">No Followers</svelte:fragment>
    </MemberList>

    <div class="divider divider-horizontal"></div>

    <MemberList members={data.following} let:id>
        {#if data.self}
            <button class="btn btn-ghost" on:click={() => unfollow(id)}>Unfollow</button>
        {/if}
        <svelte:fragment slot="fallback">No Following</svelte:fragment>
    </MemberList>
    
    <div class="divider divider-horizontal"></div>

    {#if data.self}
        <MemberList members={data.requests} let:id>
            <button class="btn btn-ghost" on:click={() => accept(id)}>Accept</button>
            <button class="btn btn-ghost" on:click={() => remove(id)}>Remove</button>
            <svelte:fragment slot="fallback">No Requests</svelte:fragment>
        </MemberList>
    {/if}

</div>
