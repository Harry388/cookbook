<script lang="ts">

    import Info from '$lib/components/community/info.svelte';

    export let data;

</script>

{#each data.communities as community}
    <div class="mt-5"></div>
    <a href="/community/{community.id}"><Info {community} /></a>
{/each}
