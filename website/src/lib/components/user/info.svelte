<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import { followUser, removeFollower } from '$lib/app/follow';
    import { invalidate } from '$app/navigation';
    import type { User } from '$lib/app/user';

    export let user: User;
    export let id: number;

    $: thisUser = id == user.id;
    $: followMessage = user.is_following ? 'Unfollow' : user.is_requested ? 'Requested' : 'Follow';

    async function toggleFollow() {
        if (user.is_following || user.is_requested) {
            await removeFollower(id, user.id).run();
        }
        else {
            await followUser(id, user.id).run();
        }
        invalidate('app:user'); // rerun user load
    }

</script>

<div class="card w-full bg-base-100 shadow-xl">
    <div class="card-body">
        <div class="flex flex-col lg:flex-row gap-y-2 gap-x-5">
            <div class="flex-1 flex gap-x-5 mb-1">
                <ProfilePic {user}/>
                <div class="flex flex-col gap-y-1">
                    <h2 class="card-title text-3xl">{ user.display_name }</h2>
                    <h2 class="card-title">@{ user.username }</h2>
                </div>
            </div>
            <div class="flex-1">
                <div class="flex lg:gap-x-10 mb-5">
                    <a class="flex-2 lg:flex-1 font-semibold text-xl" href="/user/{user.id}/follow">{ user.followers } Followers</a>
                    <div class="flex-1 lg:flex-2"></div>
                    <a class="flex-2 lg:flex-1 font-semibold text-xl" href="/user/{user.id}/follow">{ user.following } Following</a>
                </div>
            </div>
            <div class="flex-1">
                {#if thisUser}
                    <a class="btn btn-outline w-full" href="/settings">Edit Profile</a>
                {:else}
                    <button class="btn btn-outline w-full" on:click={toggleFollow}>{ followMessage }</button>
                {/if}
            </div>
        </div>
        {#if user.bio }
            <p>{ user.bio }</p>
        {/if}
    </div>
</div>
