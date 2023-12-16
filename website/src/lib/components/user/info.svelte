<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import { getContext } from 'svelte';
    import { getUser } from '$lib/app/user';
    import { followUser, removeFollower } from '$lib/app/follow';
    import type { User } from '$lib/app/user';
    import type { Writable } from 'svelte/store';

    const id: Writable<number> = getContext('id');
    const user: Writable<User> = getContext('user');

    $: self = $id == $user.id;

    async function toggleFollow() {
        if ($user.is_following) {
            await removeFollower($id, $user.id);
        }
        else {
            await followUser($id, $user.id);
        }
        $user = await getUser($user.id);
    }

</script>

<div class="card w-full bg-base-100 shadow-xl">
    <div class="card-body">
        <div class="flex flex-col lg:flex-row gap-y-2">
            <div class="flex flex-1 gap-x-5 mb-1">
                <ProfilePic user={$user}/>
                <div class="flex flex-col gap-y-1">
                    <h2 class="card-title text-3xl">{ $user.display_name }</h2>
                    <h2 class="card-title">@{ $user.username }</h2>
                </div>
            </div>
            <a class="flex-1 font-semibold text-xl" href={`/user/${$user.id}/follow`}>{ $user.followers } Followers</a>
            <a class="flex-1 font-semibold text-xl" href={`/user/${$user.id}/follow`}>{ $user.following } Following</a>
            {#if self}
                <a class="btn btn-outline" href="/editprofile">Edit Profile</a>
            {:else}
                <button class="btn btn-outline" on:click={toggleFollow}>{ $user.is_following ? 'Following' : 'Follow' }</button>
            {/if}
        </div>
        {#if $user.bio }
            <p>{ $user.bio }</p>
        {/if}
    </div>
</div>