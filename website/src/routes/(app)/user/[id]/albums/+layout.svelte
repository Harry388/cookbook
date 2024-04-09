<script lang="ts">

    import { page } from '$app/stores';

    export let data;

</script>

<div class="flex gap-x-5">

    <div class="hidden lg:flex flex-col items-center h-fit w-fit p-4 -ml-6 bg-base-300 rounded flex-none">
        {#if data.self}
            <a href="/user/{data.user.id}/albums/create" class="mb-5 btn btn-outline">Create Album</a>
        {/if}
        {#each data.albums as album}
            <a href="/user/{data.user.id}/albums/{album.id}" class="btn btn-ghost {$page.data.album?.id == album.id ? 'btn-active' : ''}">
                { album.title }
            </a>
        {/each}
    </div>

    <div class="w-full">
        <slot />
    </div>

</div>
