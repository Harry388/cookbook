<script lang="ts">

    import Info from '$lib/components/community/info.svelte';

    export let data;

</script>

<div class="lg:hidden">
    <a href="/community/create" class="mb-5 btn btn-outline w-full">Create Community</a>
    {#each data.communities as community}
        <div class="mt-5"></div>
        <a href="/community/{community.id}"><Info {community} options={false} /></a>
    {/each}
</div>
