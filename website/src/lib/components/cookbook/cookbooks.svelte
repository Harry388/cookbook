<script lang="ts">

    import CookbookLink from '$lib/components/cookbook/cookbookLink.svelte';
    import type { Cookbook } from '$lib/app/cookbook';

    export let cookbooks: Cookbook[];

</script>

{#each cookbooks as cookbook}
    <div class="mt-5"></div>
    <CookbookLink {cookbook} />
{/each}
