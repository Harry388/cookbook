<script lang="ts">

    import SearchList from '$lib/components/util/searchList.svelte';
    import SearchItem from '$lib/components/util/searchItem.svelte';
    import ProfilePic from '$lib/components/user/profilePic.svelte';

    export let members: {
        id: number,
        username: string,
        display_name: string
    }[];
    export let title: string;

</script>
        

<h3 class="font-bold text-lg">{ title }</h3>

{#if members.length}
    <SearchList>
        <table class="table w-full">
            <tbody>
                {#each members as member}
                    <SearchItem key={member.display_name}>
                        <tr class="w-full">
                            <td>
                                <a class="w-full flex items-center gap-3" href={`/user/${member.id}`}>
                                    <ProfilePic user={member} />
                                    <div>
                                        <div class="font-bold">{ member.display_name }</div>
                                        <div class="opacity-50">@{ member.username }</div>
                                    </div>
                                </a>
                            </td>
                            <td>
                                <slot id={member.id} />
                            </td>
                        </tr>
                    </SearchItem>
                {/each}
            </tbody>
        </table>
    </SearchList>
{:else}
    <slot name="fallback" />
{/if}
