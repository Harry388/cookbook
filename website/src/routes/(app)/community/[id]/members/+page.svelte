<script lang="ts">

    import MemberList from '$lib/components/user/memberList.svelte';
    import SearchList from '$lib/components/util/searchList.svelte';
    import SearchItem from '$lib/components/util/searchItem.svelte';
    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import { acceptMember, leaveCommunity } from '$lib/app/community';
    import { updateCommunityUser } from '$lib/app/communityMember.js';
    import { invalidate } from '$app/navigation';

    export let data;

    let tab = 0;

    $: admins = data.members.filter(m => m.permission == 'ADMIN');
    $: users = data.members.filter(m => m.permission == 'USER');

    async function leave(userId: number) {
        const response = await leaveCommunity(data.community.id, userId).run();
        if (response.ok) {
            invalidate('app:communityMembers');
            invalidate('app:community');
        }
    }

    async function setPermission(userId: number, permission: 'ADMIN' | 'USER') {
        const response = await updateCommunityUser(data.community.id, userId, permission).run();
        if (response.ok) {
            invalidate('app:communityMembers');
            invalidate('app:community');
        }
    }

    async function accept(userId: number) {
        const response = await acceptMember(data.community.id, userId).run();
        if (response.ok) {
            invalidate('app:communityMembers');
            invalidate('app:community');
        }
    }

</script>

<div role="tablist" class="my-5 tabs tabs-bordered tabs-lg lg:hidden">
    <button role="tab" class="tab {(tab == 0) && 'tab-active'}" on:click={() => tab = 0}>
        Admins
    </button>
    <button role="tab" class="tab {(tab == 1) && 'tab-active'}" on:click={() => tab = 1}>
        Members
    </button>
    {#if data.community.is_admin}
        <button role="tab" class="tab {(tab == 2) && 'tab-active'}" on:click={() => tab = 2}>
            Requests
        </button>
    {/if}
</div>

<div class="block lg:flex">

    <div class="{(tab != 0) && 'hidden'} lg:block lg:w-1/3">
        <MemberList members={admins} let:id title="Admins">
        {#if data.community.is_admin}
            {#if data.id != id }
                <button class="btn btn-ghost" on:click={() => setPermission(id, 'USER')}>Demote</button>
            {/if}
            <button class="btn btn-ghost" on:click={() => leave(id)}>Remove</button>
        {/if}
            <svelte:fragment slot="fallback">No Admins</svelte:fragment>
        </MemberList>
    </div>

    <div class="divider divider-horizontal hidden lg:flex"></div>

    <div class="{(tab != 1) && 'hidden'} lg:block lg:w-1/3">
        <MemberList members={users} let:id title="Members">
            {#if data.community.is_admin}
                <button class="btn btn-ghost" on:click={() => setPermission(id, 'ADMIN')}>Promote</button>
                <button class="btn btn-ghost" on:click={() => leave(id)}>Remove</button>
            {/if}
            <svelte:fragment slot="fallback">No Members</svelte:fragment>
        </MemberList>
    </div>

    {#if data.community.is_admin}
        <div class="divider divider-horizontal hidden lg:flex"></div>

        <div class="{(tab != 2) && 'hidden'} lg:block lg:w-1/3">
            <MemberList members={data.requests} let:id title="Requests">
                <button class="btn btn-ghost" on:click={() => accept(id)}>Accept</button>
                <button class="btn btn-ghost" on:click={() => leave(id)}>Remove</button>
                <svelte:fragment slot="fallback">No Requests</svelte:fragment>
            </MemberList>
        </div>
    {/if}

</div>
