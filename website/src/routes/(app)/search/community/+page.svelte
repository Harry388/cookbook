<script lang="ts">

    import Info from '$lib/components/community/info.svelte';

    export let data;

</script>

{#if data.communities.length == 0}
    <p class="text-lg text-center">No communities found</p>
{/if}
{#each data.communities as community}
    <div class="mt-5"></div>
    <a href="/community/{community.id}"><Info {community} /></a>
{/each}
