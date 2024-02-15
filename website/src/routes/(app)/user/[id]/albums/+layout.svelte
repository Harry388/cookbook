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
            {#if data.self}
                <li><a href="/user/{data.user.id}/albums/create" class="btn btn-outline">Create Album</a></li>
            {/if}
            {#each data.albums as album}
                <li><a href="/user/{data.user.id}/albums/{album.id}" class={$page.data.album?.id == album.id ? 'active' : ''}>{ album.title }</a></li>
            {/each}
        </ul>
    </div>
</div>
