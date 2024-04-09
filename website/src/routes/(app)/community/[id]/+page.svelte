<script lang="ts">

    import Info from '$lib/components/community/info.svelte';
    import Post from '$lib/components/post/post.svelte';
    import Confirm from '$lib/components/util/confirm.svelte';
    import { removePost } from '$lib/app/community';
    import { invalidate } from '$app/navigation';

    export let data;

    async function remove(id: number) {
        const response = await removePost(data.community.id, id).run();
        if (response.ok) {
            invalidate('app:community');
        }
    }

</script>

<Info community={data.community} />

{#if data.community.public || data.community.is_member}
    <div class="lg:w-5/12 lg:m-auto flex flex-col">
        {#if data.community.is_member}
            <a href="/post/create?c={data.community.id}" class="mt-5 btn btn-outline"><i class="fa-solid fa-plus"></i>Create Post</a>
        {/if}
        {#each data.posts as post}
            <div class="mt-5"></div>
            <Post {post} link />
            {#if data.community.is_admin }
                <Confirm let:show on:confirm={() => remove(post.id)} id={post.id}>
                    <button class="btn btn-outline btn-error mt-5 w-full" on:click={show}>Remove Post from Community</button> 
                </Confirm>
            {/if}
        {/each}
    </div>
{:else}
    <div class="w-fit py-24 m-auto">Community is private. Request to join this community to see its contents.</div>
{/if}


