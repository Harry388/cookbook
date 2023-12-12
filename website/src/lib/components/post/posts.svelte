<script lang="ts">

    import PostPreview from '$lib/components/post/postPreview.svelte';
    import type { Post } from '$lib/app/post';

    export let posts: Post[];

</script>

<div class="mt-5 flex gap-5 flex-col items-center">
    {#each posts as post}
        <PostPreview {post} />
    {/each}
</div>