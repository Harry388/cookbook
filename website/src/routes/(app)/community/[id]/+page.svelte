<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';
    import { setContext } from 'svelte';
    import { writable } from 'svelte/store';
    import type { Post } from '$lib/app/post';
    import { leaveCommunity, joinCommunity, getCommunity } from '$lib/app/community';

    export let data;

    let community = data.community;
    $: community = data.community;

    const posts = writable<Post[]>();
    $: posts.set(data.posts);
    setContext('posts', posts);

    async function toggleMember() {
        if (community.is_member) {
            await leaveCommunity(community.id, data.id);
        }
        else {
            await joinCommunity(community.id);
        }
        community = await getCommunity(community.id);
    }

</script>

<div class="card w-full bg-base-100 shadow-xl mb-5">
    <div class="card-body">
        <div class="flex flex-col lg:flex-row gap-y-2 gap-x-5">
            <div class="flex-1 flex gap-x-5 mb-1">
                <!-- <ProfilePic user={$user}/> -->
                <div class="flex flex-col gap-y-1">
                    <h2 class="card-title text-3xl">{ community.title }</h2>
                </div>
            </div>
            <div class="flex-1">
                <div class="flex lg:gap-x-10 mb-5">
                    <a href="/community/{community.id}/members" class="flex-2 lg:flex-1 font-semibold text-xl">{ community.users } Members</a>
                </div>
            </div>
            <div class="flex-1">
                <button on:click={toggleMember} class="btn btn-outline w-full">{ community.is_member ? 'Leave' : 'Join' }</button>
                {#if community.is_admin} 
                    <a href="/community/{community.id}/edit" class="btn btn-outline w-full">Edit Community</a>
                {/if}
            </div>
        </div>
        {#if community.description }
            <p>{ community.description }</p>
        {/if}
    </div>
</div>

<a href="/post/create?c={community.id}" class="btn btn-outline">Create Post</a>

<Posts />
