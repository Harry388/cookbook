<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';
    import { setContext } from 'svelte';
    import { writable } from 'svelte/store';
    import type { Post } from '$lib/app/post';

    export let data;

    const posts = writable<Post[]>();
    $: posts.set(data.posts);
    setContext('posts', posts);

</script>

{#if data.self}
    <a href="/post/create" class="btn btn-outline">Create Post</a>
{/if}

<Posts />