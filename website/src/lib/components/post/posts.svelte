<script lang="ts">

    import PostPreview from '$lib/components/post/postPreview.svelte';
    import { getContext } from 'svelte';
    import type { Writable } from 'svelte/store';
    import type { Post } from '$lib/app/post';

    const posts: Writable<Post[]> = getContext('posts');

</script>

<div class="mt-5 flex gap-5 flex-col items-center">
    {#each $posts as post}
        <PostPreview {post} link />
    {/each}
</div>