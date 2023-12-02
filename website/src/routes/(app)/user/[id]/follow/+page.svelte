<script>

    export let data;

</script>

<div>Followers</div>
{#each data.followers as follower}
    { follower.display_name }
{/each}
<div>Following</div>
{#each data.following as following}
    { following.display_name }
{/each}