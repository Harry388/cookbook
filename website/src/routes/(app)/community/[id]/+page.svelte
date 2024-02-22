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

{#if data.community.is_member}
    <a href="/post/create?c={data.community.id}" class="btn btn-outline">Create Post</a>
{/if}

{#if data.community.public || data.community.is_member}
    <div class="w-11/12 lg:w-1/3 m-auto">
        {#each data.posts as post}
            <div class="mt-5"></div>
            <Post {post} link />
            {#if data.community.is_admin }
                <button class="btn btn-outline" on:click={() => remove(post.id)}>Remove</button> 
            {/if}
        {/each}
    </div>
{:else}
    Community is private
{/if}


