<script lang="ts">

    import { goto } from '$app/navigation';

    const routes = ['posts', 'recipes', 'communities', 'tags', 'users'];

    export let data;

    let text = data.search || '';

    $: passText = text ? `?s=${text}` : '';

    function search() {
        if (text) {
            goto(`?s=${text}`);
        }
    }

</script>

<form class="form-control lg:w-1/2 lg:m-auto" on:submit|preventDefault={search}>
    <label for="#search" class="label">
        <span class="label-text">Search</span>
    </label>
    <div id="search" class="flex gap-x-5 items-center ">
        <input type="text" min="1" bind:value={text} placeholder="Search" class="flex-grow input input-bordered" />
        <button class="fa-solid fa-magnifying-glass btn btn-ghost text-2xl"><input type="submit" value="" /></button>
    </div>
</form>

<div role="tablist" class="my-5 tabs tabs-bordered tabs-lg">
    {#each routes as route}
        <a href="/search/{route}/{passText}" role="tab" class="tab {(data.path == route) && 'tab-active'}">
            { route.charAt(0).toUpperCase() + route.substring(1) }
        </a>
    {/each}
</div>

<div class="w-11/12 lg:w-1/3 m-auto">
    <slot/>
</div>
