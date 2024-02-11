<script lang="ts">

    import { page } from '$app/stores';

    export let data;

</script>

<div class="drawer lg:drawer-open">
    <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content flex flex-col items-center bg-base-200 h-full">
        <!-- Page content here -->
        <div class="h-full w-full p-5">
            <slot />
        </div>
    </div> 
    <div class="drawer-side">
        <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label> 
        <ul class="menu flex p-4 w-40 min-h-full bg-base-300 text-base-content">
            <li><a href="/community/create" class="btn btn-outline">Create Community</a></li>
            {#each data.communities as community}
                <li><a href="/community/{community.id}" class={$page.data.community?.id == community.id ? 'active' : ''}>{ community.title }</a></li>
            {/each}
        </ul>
    </div>
</div>
