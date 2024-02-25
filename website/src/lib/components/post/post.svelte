<script lang="ts">

    import Entry from '$lib/components/entries/entry.svelte';
    import Media from '$lib/components/post/media.svelte';
    import type { Post } from '$lib/app/post';

    export let post: Post;
    export let link = false;

    $: media = post.media ? post.media.filter(m => m != null) : [];

</script>

<Entry entry={post} {link} type="post">
    <svelte:fragment slot="media">
    {#if media.length }
        <Media {media} />
        <div class="pb-1"></div>
    {/if}
    </svelte:fragment>
</Entry>
