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

<div class="flex justify-center">

    <div class="overflow-x-auto w-1/3">

        <h3 class="font-bold text-lg">Followers</h3>

        {#if $followers.length}

            <table class="table">
                <tbody>
                    {#each $followers as follower}
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
        
        {#if $following.length}

            <table class="table">
                <tbody>
                    {#each $following as following}
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
                        </tr>
                    {/each}
                </tbody>
            </table>

        {:else}

            <div>No Following</div>

        {/if}

    </div>

</div>