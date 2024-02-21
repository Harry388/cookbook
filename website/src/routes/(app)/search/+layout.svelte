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

<form class="form-control" on:submit|preventDefault={search}>
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Search</span>
    </label>
    <input type="text" min="1" bind:value={text} placeholder="Search" class="input input-bordered" />
    <a class="btn btn-primary w-fit mt-5" href={passText}>Search</a>
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
