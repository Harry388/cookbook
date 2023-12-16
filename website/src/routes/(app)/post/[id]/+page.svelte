<script lang="ts">

    import PostPreview from '$lib/components/post/postPreview.svelte';
    import { deletePost } from '$lib/app/post';
    import { goto } from '$app/navigation';

    export let data;

    $: ownsPost = data.post.user_id == data.id;

    async function onDelete() {
        if (!confirm('Are you sure?')) return;
        const response = await deletePost(data.post.id);
        if (response.ok) {
            goto('/user');
        }
    }

</script>

<a class="btn btn-outline" href="/user/{data.post.user_id}">Back</a>

{#if ownsPost}
    <button class="btn btn-error" on:click={onDelete}>Delete Post</button>
{/if}

<div class="flex flex-col items-center lg:flex-row  mt-5">

    <PostPreview post={data.post} />

</div>

