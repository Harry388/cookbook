<script lang="ts">

    import Info from '$lib/components/community/info.svelte';
    import Post from '$lib/components/post/post.svelte';
    import { removePost } from '$lib/app/community';
    import { invalidate } from '$app/navigation';

    export let data;

    async function remove(id: number) {
        if (!confirm('Are you sure?')) return;
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
            <div class="flex gap-x-5">
                <Post {post} link />
                {#if data.community.is_admin }
                    <button class="fa-regular fa-trash-can text-2xl btn" on:click={() => remove(post.id)}></button> 
                {/if}
            </div>
        {/each}
    </div>
{:else}
    Community is private
{/if}


