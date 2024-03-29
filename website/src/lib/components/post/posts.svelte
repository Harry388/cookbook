<script lang="ts">

    import PostComponent from '$lib/components/post/post.svelte';
    import type { Post } from '$lib/app/post';

    export let posts: Post[];

</script>

{#each posts as post (post.id)}
    <div class="mt-5"></div>
    <PostComponent {post} link />
{/each}
