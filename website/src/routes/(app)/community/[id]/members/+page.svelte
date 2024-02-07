<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import { leaveCommunity } from '$lib/app/community';
    import { getCommunityMembers, updateCommunityUser } from '$lib/app/communityMember';

    export let data;

    let members = data.members;
    $: admins = members.filter(m => m.permission == 'ADMIN');
    $: users = members.filter(m => m.permission == 'USER');

    async function leave(userId: number) {
        const response = await leaveCommunity(data.community.id, userId);
        if (!response.ok) return;
        members = await getCommunityMembers(data.community.id);
    }

    async function update(userId: number, permission: 'ADMIN' | 'USER') {
        const response = await updateCommunityUser(data.community.id, userId, permission);
        if (!response.ok) return;
        members = await getCommunityMembers(data.community.id);
    }


</script>

<div class="flex justify-center">

    <div class="overflow-x-auto w-1/3">

        <h3 class="font-bold text-lg">Admins</h3>

        {#if admins.length}

            <table class="table">
                <tbody>
                    {#each admins as admin}
                        <tr>
                            <td>
                                <a class="flex items-center gap-3" href={`/user/${admin.id}`}>
                                    <ProfilePic user={admin} />
                                    <div>
                                        <div class="font-bold">{ admin.display_name }</div>
                                        <div class="opacity-50">@{ admin.username }</div>
                                    </div>
                                </a>
                            </td>
                            <th>
                                {#if data.community.is_admin}
                                    {#if admin.id != data.id}
                                        <button class="btn btn-ghost" on:click={() => update(admin.id, 'USER')}>Demote</button>
                                    {/if}
                                    <button class="btn btn-ghost" on:click={() => leave(admin.id)}>Remove</button>
                                {/if}
                            </th>
                        </tr>
                    {/each}
                </tbody>
            </table>

        {:else}

            <div>No Admins</div>

        {/if}

    </div>

    <div class="divider divider-horizontal"></div>
    
    <div class="overflow-x-auto w-1/3">

        <h3 class="font-bold text-lg">Users</h3>
        
        {#if users.length}

            <table class="table">
                <tbody>
                    {#each users as user}
                        <tr>
                            <td>
                                <a class="flex items-center gap-3" href={`/user/${user.id}`}>
                                    <ProfilePic user={user} />
                                    <div>
                                        <div class="font-bold">{ user.display_name }</div>
                                        <div class="opacity-50">@{ user.username }</div>
                                    </div>
                                </a>
                            </td>
                            <th>
                                {#if data.community.is_admin}
                                    {#if user.id != data.id}
                                        <button class="btn btn-ghost" on:click={() => update(user.id, 'ADMIN')}>Premote</button>
                                    {/if}
                                    <button class="btn btn-ghost" on:click={() => leave(user.id)}>Remove</button>
                                {/if}
                            </th>
                        </tr>
                    {/each}
                </tbody>
            </table>

        {:else}

            <div>No Users</div>

        {/if}

    </div>

</div>
