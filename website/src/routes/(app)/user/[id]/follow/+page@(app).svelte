<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import { removeFollower } from '$lib/app/follow';
    import { invalidate } from '$app/navigation';

    export let data;

    async function remove(followerId: number) {
        const response = await removeFollower(followerId, data.id).run();
        if (response.ok) {
            invalidate('app:userFollow');
        }
    }

    async function unfollow(followingId: number) {
        const response = await removeFollower(data.id, followingId).run();
        if (response.ok) {
            invalidate('app:userFollow');
        }
    }

</script>

<div class="flex justify-center">

    <div class="overflow-x-auto w-1/3">

        <h3 class="font-bold text-lg">Followers</h3>

        {#if data.followers.length}

            <table class="table">
                <tbody>
                    {#each data.followers as follower}
                        <tr>
                            <td>
                                <a class="flex items-center gap-3" href={`/user/${follower.id}`}>
                                    <ProfilePic user={follower} />
                                    <div>
                                        <div class="font-bold">{ follower.display_name }</div>
                                        <div class="opacity-50">@{ follower.username }</div>
                                    </div>
                                </a>
                            </td>
                            <th>
                                {#if data.self}
                                    <button class="btn btn-ghost" on:click={() => remove(follower.id)}>Remove</button>
                                {/if}
                            </th>
                        </tr>
                    {/each}
                </tbody>
            </table>

        {:else}

            <div>No Followers</div>

        {/if}

    </div>

    <div class="divider divider-horizontal"></div>
    
    <div class="overflow-x-auto w-1/3">

        <h3 class="font-bold text-lg">Following</h3>
        
        {#if data.following.length}

            <table class="table">
                <tbody>
                    {#each data.following as following}
                        <tr>
                            <td>
                                <a class="flex items-center gap-3" href={`/user/${following.id}`}>
                                    <ProfilePic user={following} />
                                    <div>
                                        <div class="font-bold">{ following.display_name }</div>
                                        <div class="opacity-50">@{ following.username }</div>
                                    </div>
                                </a>
                            </td>
                            <th>
                                {#if data.self}
                                    <button class="btn btn-ghost" on:click={() => unfollow(following.id)}>Unfollow</button>
                                {/if}
                            </th>
                        </tr>
                    {/each}
                </tbody>
            </table>

        {:else}

            <div>No Following</div>

        {/if}

    </div>

</div>
