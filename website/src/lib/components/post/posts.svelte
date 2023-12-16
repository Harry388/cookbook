<script lang="ts">

    import PostPreview from '$lib/components/post/postPreview.svelte';
    import { getContext } from 'svelte';
    import { deletePost, getUserPosts } from '$lib/app/post';
    import type { Writable } from 'svelte/store';
    import type { Post } from '$lib/app/post';
    import type { User } from '$lib/app/user';

    const posts: Writable<Post[]> = getContext('posts');
    const user: Writable<User> = getContext('user');

    async function onDelete(id: number) {
        const response = await deletePost(id);
        if (response.ok) {
            $posts = await getUserPosts($user.id);
        }
    }

</script>

<div class="mt-5 flex gap-5 flex-col items-center">
    {#each $posts as post}
        <PostPreview {post} on:delete={() => onDelete(post.id)} />
    {/each}
</div>