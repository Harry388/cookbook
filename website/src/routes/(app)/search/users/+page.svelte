<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';

    export let data;

</script>

{#if data.users.length == 0}
    <p class="text-lg text-center">No users found</p>
{/if}
{#each data.users as user}
    <div class="mt-5"></div>
    <a class="flex items-center gap-3" href={`/user/${user.id}`}>
        <ProfilePic {user} />
        <div>
            <div class="font-bold">{ user.display_name }</div>
            <div class="opacity-50">@{ user.username }</div>
        </div>
    </a>
{/each}
