<script lang="ts">

    import MemberList from '$lib/components/user/memberList.svelte';
    import { acceptFollow, removeFollower } from '$lib/app/follow';
    import { invalidate } from '$app/navigation';

    export let data;

    let tab = 0;

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

<div role="tablist" class="my-5 tabs tabs-bordered tabs-lg lg:hidden">
    <button role="tab" class="text-sm lg:text-lg tab {(tab == 0) && 'tab-active'}" on:click={() => tab = 0}>
        Followers
    </button>
    <button role="tab" class="text-sm lg:text-lg tab {(tab == 1) && 'tab-active'}" on:click={() => tab = 1}>
        Following
    </button>
    {#if data.self}
        <button role="tab" class="text-sm lg:text-lg tab {(tab == 2) && 'tab-active'}" on:click={() => tab = 2}>
            Requests
        </button>
    {/if}
</div>

<div class="block lg:flex">

    <div class="{(tab != 0) && 'hidden'} lg:block lg:w-1/3">
        <MemberList members={data.followers} let:id>
            {#if data.self}
                <button class="btn btn-ghost" on:click={() => remove(id)}>Remove</button>
            {/if}
            <svelte:fragment slot="fallback">No Followers</svelte:fragment>
        </MemberList>
    </div>

    <div class="divider divider-horizontal hidden lg:flex"></div>

    <div class="{(tab != 1) && 'hidden'} lg:block lg:w-1/3">
        <MemberList members={data.following} let:id>
            {#if data.self}
                <button class="btn btn-ghost" on:click={() => unfollow(id)}>Unfollow</button>
            {/if}
            <svelte:fragment slot="fallback">No Following</svelte:fragment>
        </MemberList>
    </div>
    

    {#if data.self}
        <div class="divider divider-horizontal hidden lg:flex"></div>

        <div class="{(tab != 2) && 'hidden'} lg:block lg:w-1/3">
            <MemberList members={data.requests} let:id>
                <button class="btn btn-ghost" on:click={() => accept(id)}>Accept</button>
                <button class="btn btn-ghost" on:click={() => remove(id)}>Remove</button>
                <svelte:fragment slot="fallback">No Requests</svelte:fragment>
            </MemberList>
        </div>
    {/if}

</div>
