<script>

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import { remove } from '$lib/apiFetch';

    export let data;

    async function removeFollower(followerId) {
        await remove(`user/${followerId}/unfollow/${data.id}`).run();
    }

</script>

<a class="btn btn-outline" href={`/user/${data.userId}`}>Back</a>

<h3 class="font-bold text-lg">Followers</h3>
{#each data.followers as follower}
    <a href={`/user/${follower.id}`}><ProfilePic user={follower}/> { follower.display_name } </a>
    {#if data.self}
        <button class="btn btn-outline" on:click={() => removeFollower(follower.id)}>Remove</button>
    {/if}
{/each}

<h3 class="font-bold text-lg">Following</h3>
{#each data.following as following}
    <a href={`/user/${following.id}`}><ProfilePic user={following}/> { following.display_name } </a>
{/each}