<script lang="ts">

    import Entry from '$lib/components/entries/entry.svelte';
    import Media from '$lib/components/post/media.svelte';
    import type { Post } from '$lib/app/post';

    export let post: Post;
    export let link = false;

</script>

<Entry entry={post} {link} type="post">
    <svelte:fragment slot="media">
    {#if !(-1 in post.media) }
        <Media media={post.media} />
        <div class="pb-1"></div>
    {/if}
    </svelte:fragment>
</Entry>
