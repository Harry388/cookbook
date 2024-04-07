<script lang="ts">

    import SearchList from '$lib/components/util/searchList.svelte';
    import SearchItem from '$lib/components/util/searchItem.svelte';
    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import { acceptFollow, removeFollower } from '$lib/app/follow';
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

    async function accept(userId: number) {
        const response = await acceptFollow(userId).run();
        if (response.ok) {
            invalidate('app:userFollow');
        }
    }

</script>

<div class="flex justify-center">

    <div class="overflow-x-auto w-1/3">

        <h3 class="font-bold text-lg">Followers</h3>

        {#if data.followers.length}

            <SearchList>

            <table class="table">
                <tbody>
                    {#each data.followers as follower}

                        <SearchItem key={follower.display_name}>

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


                        </SearchItem>

                    {/each}
                </tbody>
            </table>

            </SearchList>

        {:else}

            <div>No Followers</div>

        {/if}

    </div>

    <div class="divider divider-horizontal"></div>
    
    <div class="overflow-x-auto w-1/3">

        <h3 class="font-bold text-lg">Following</h3>
        
        {#if data.following.length}

            <SearchList>

            <table class="table">
                <tbody>
                    {#each data.following as following}

                        <SearchItem key={following.display_name}>

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

                        </SearchItem>

                    {/each}
                </tbody>
            </table>

            </SearchList>

        {:else}

            <div>No Following</div>

        {/if}

    </div>

    {#if data.self}

        <div class="divider divider-horizontal"></div>
        
        <div class="overflow-x-auto w-1/3">

            <h3 class="font-bold text-lg">Requests</h3>
            
            {#if data.requests.length}

                <SearchList>

                <table class="table">
                    <tbody>
                        {#each data.requests as request}

                            <SearchItem key={request.display_name}>

                            <tr>
                                <td>
                                    <a class="flex items-center gap-3" href={`/user/${request.id}`}>
                                        <ProfilePic user={request} />
                                        <div>
                                            <div class="font-bold">{ request.display_name }</div>
                                            <div class="opacity-50">@{ request.username }</div>
                                        </div>
                                    </a>
                                </td>
                                <th>
                                    <button class="btn btn-ghost" on:click={() => accept(request.id)}>Accept</button>
                                    <button class="btn btn-ghost" on:click={() => remove(request.id)}>Remove</button>
                                </th>
                            </tr>

                            </SearchItem>

                        {/each}
                    </tbody>
                </table>

                </SearchList>

            {:else}

                <div>No Requests</div>

            {/if}

        </div>
    
    {/if}

</div>
