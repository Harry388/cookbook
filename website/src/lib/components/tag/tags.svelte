<script lang="ts">

    import TagComponent from '$lib/components/tag/tag.svelte';
    import type { Tag } from '$lib/app/tag';

    export let tags: Tag[];

</script>

<div class="flex gap-2 flex-wrap">
    {#each tags as tag}
        <TagComponent {tag} />
    {/each}
</div>
