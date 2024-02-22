<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import { acceptMember, leaveCommunity } from '$lib/app/community';
    import { updateCommunityUser } from '$lib/app/communityMember.js';
    import { invalidate } from '$app/navigation';

    export let data;

    $: admins = data.members.filter(m => m.permission == 'ADMIN');
    $: users = data.members.filter(m => m.permission == 'USER');

    async function leave(userId: number) {
        const response = await leaveCommunity(data.community.id, userId).run();
        if (response.ok) {
            invalidate('app:communityMembers');
            invalidate('app:community');
        }
    }

    async function setPermission(userId: number, permission: 'ADMIN' | 'USER') {
        const response = await updateCommunityUser(data.community.id, userId, permission).run();
        if (response.ok) {
            invalidate('app:communityMembers');
            invalidate('app:community');
        }
    }

    async function accept(userId: number) {
        const response = await acceptMember(data.community.id, userId).run();
        if (response.ok) {
            invalidate('app:communityMembers');
            invalidate('app:community');
        }
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
                                <a class="flex items-center gap-3" href="/user/{admin.id}">
                                    <ProfilePic user={admin} />
                                    <div>
                                        <div class="font-bold">{ admin.display_name }</div>
                                        <div class="opacity-50">@{ admin.username }</div>
                                    </div>
                                </a>
                            </td>
                            <th>
                                {#if data.community.is_admin}
                                    {#if data.id != admin.id }
                                        <button class="btn btn-ghost" on:click={() => setPermission(admin.id, 'USER')}>Demote</button>
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

        <h3 class="font-bold text-lg">Members</h3>
        
        {#if users.length}

            <table class="table">
                <tbody>
                    {#each users as user}
                        <tr>
                            <td>
                                <a class="flex items-center gap-3" href="/user/{user.id}">
                                    <ProfilePic user={user} />
                                    <div>
                                        <div class="font-bold">{ user.display_name }</div>
                                        <div class="opacity-50">@{ user.username }</div>
                                    </div>
                                </a>
                            </td>
                            <th>
                                {#if data.community.is_admin}
                                    <button class="btn btn-ghost" on:click={() => setPermission(user.id, 'ADMIN')}>Promote</button>
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

    {#if data.community.is_admin}
        <div class="divider divider-horizontal"></div>
        
        <div class="overflow-x-auto w-1/3">

            <h3 class="font-bold text-lg">Requests</h3>
            
            {#if data.requests.length}

                <table class="table">
                    <tbody>
                        {#each data.requests as request}
                            <tr>
                                <td>
                                    <a class="flex items-center gap-3" href="/user/{request.id}">
                                        <ProfilePic user={request} />
                                        <div>
                                            <div class="font-bold">{ request.display_name }</div>
                                            <div class="opacity-50">@{ request.username }</div>
                                        </div>
                                    </a>
                                </td>
                                <th>
                                    {#if data.community.is_admin}
                                        <button class="btn btn-ghost" on:click={() => accept(request.id)}>Accept</button>
                                        <button class="btn btn-ghost" on:click={() => leave(request.id)}>Remove</button>
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
    {/if}

</div>
