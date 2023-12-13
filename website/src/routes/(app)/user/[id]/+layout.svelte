<script>

    import Info from '$lib/components/user/info.svelte';

    export let data;

    const routes = ['posts', 'other'];

</script>

<Info user={data.user} self={data.self} />

<div role="tablist" class="my-5 tabs tabs-bordered tabs-lg">
    {#each routes as route}
        <a href={`/user/${data.id}/${route}`} role="tab" class={`tab ${(data.path == route) && 'tab-active'}`}>
            { route.charAt(0).toUpperCase() + route.substring(1) }
        </a>
    {/each}
</div>

<slot/>