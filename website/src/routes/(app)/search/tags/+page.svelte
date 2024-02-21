<script lang="ts">

    import Tag from '$lib/components/tag/tag.svelte';

    export let data;

</script>

{#each data.tags as tag}
    <div class="mt-5"></div>
    <Tag {tag} />
{/each}
