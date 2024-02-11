<script lang="ts">

    import PostComponent from '$lib/components/post/post.svelte';
    import type { PostFull } from '$lib/app/post';

    export let posts: PostFull[];

</script>

<div class="flex gap-5 flex-col items-center">
    {#each posts as post}
        <PostComponent {post} link />
    {/each}
</div>
