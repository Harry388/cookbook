<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import { removeFollower, getUserFollow } from '$lib/app/follow';
    import { writable } from 'svelte/store';
    import type { Follow } from '$lib/app/follow';

    export let data;

    const following = writable<Follow[]>();
    const followers = writable<Follow[]>();
    $: following.set(data.following);
    $: followers.set(data.followers);

    async function remove(followerId: number) {
        await removeFollower(followerId, data.id);
        const follow = await getUserFollow(data.id);
        $following = follow.following;
        $followers = follow.followers;
    }

</script>

<a class="btn btn-outline" href={`/user/${data.userId}`}>Back</a>

<h3 class="font-bold text-lg">Followers</h3>
{#each $followers as follower}
    <a href={`/user/${follower.id}`}><ProfilePic user={follower}/> { follower.display_name } </a>
    {#if data.self}
        <button class="btn btn-outline" on:click={() => remove(follower.id)}>Remove</button>
    {/if}
{/each}

<h3 class="font-bold text-lg">Following</h3>
{#each $following as following}
    <a href={`/user/${following.id}`}><ProfilePic user={following}/> { following.display_name } </a>
{/each}