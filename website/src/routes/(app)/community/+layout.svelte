<script lang="ts">

    import { page } from '$app/stores';

    export let data;

</script>

<div class="w-full flex gap-x-5">

    <div class="flex flex-col items-center h-fit w-fit p-4 -ml-6 bg-base-300 rounded flex-none">
        <a href="/community/create" class="mb-5 btn btn-outline">Create Community</a>
        {#each data.communities as community}
            <a href="/community/{community.id}" class="btn btn-ghost {$page.data.community?.id == community.id ? 'btn-active' : ''}">
                { community.title }
            </a>
        {/each}
    </div>

    <div class="w-full">
        <slot />
    </div>

</div>
