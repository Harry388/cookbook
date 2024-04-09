<script lang="ts">

    import { joinCommunity, leaveCommunity } from '$lib/app/community';
    import { invalidate } from '$app/navigation';
    import { getContext } from 'svelte';
    import type { Community } from '$lib/app/community';

    export let community: Community;
    export let options = true;

    const id: number = getContext('id');

    $: followMessage = community.is_member ? 'Leave' : community.is_requested ? 'Requested' : 'Join';

    async function toggleJoin() {
        if (community.is_member || community.is_requested) {
            await leaveCommunity(community.id, id).run();
        }
        else {
            await joinCommunity(community.id).run();
        }
        invalidate('app:community');
    }

</script>

<div class="card w-full bg-base-100 shadow-xl">
    <div class="card-body">
        <div class="flex flex-col lg:flex-row gap-y-2 gap-x-5">
            <div class="flex-1 flex gap-x-5 mb-1">
                <div class="flex flex-col gap-y-1">
                    <h2 class="card-title text-3xl">{ community.title }</h2>
                </div>
            </div>
            <div class="flex-1">
                <div class="flex lg:gap-x-10 mb-5">
                    <a class="flex-2 lg:flex-1 font-semibold text-xl" href="/community/{community.id}/members">{ community.users } Members</a>
                </div>
            </div>
            {#if options}
                <div class="flex-1">
                    {#if community.is_admin}
                        <a class="btn btn-outline w-full mb-5" href="/community/{community.id}/edit">Edit Community</a>
                    {/if}
                    <button class="btn btn-outline w-full" on:click={toggleJoin}>{ followMessage }</button>
                </div>
            {/if}
        </div>
        {#if community.description }
            <p>{ community.description }</p>
        {/if}
    </div>
</div>
