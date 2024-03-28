<script lang="ts">

    import TagComponent from '$lib/components/tag/tag.svelte';
    import type { Tag } from '$lib/app/tag';

    export let tags: Tag[];

</script>

<div class="flex gap-x-2">
    {#each tags as tag}
        <TagComponent {tag} />
    {/each}
</div>
