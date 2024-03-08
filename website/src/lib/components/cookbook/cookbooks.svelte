<script lang="ts">

    import CookbookLink from '$lib/components/cookbook/cookbookLink.svelte';
    import type { Cookbook } from '$lib/app/cookbook';

    export let cookbooks: Cookbook[];

</script>

{#each cookbooks as cookbook}
    <CookbookLink {cookbook} />
{/each}
