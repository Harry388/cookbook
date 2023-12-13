<script>

    import ProfilePic from '$lib/components/user/profilePic.svelte';

    export let data;

</script>

<a class="btn btn-outline" href={`/user/${data.id}`}>Back</a>

<h3 class="font-bold text-lg">Followers</h3>
{#each data.followers as follower}
    <a href={`/user/${follower.id}`}><ProfilePic user={follower}/> { follower.display_name } </a>
{/each}

<h3 class="font-bold text-lg">Following</h3>
{#each data.following as following}
    <a href={`/user/${following.id}`}><ProfilePic user={following}/> { following.display_name } </a>
{/each}