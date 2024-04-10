<script lang="ts">

    const routes = ['feed', 'explore'];

    export let data;

</script>

<div role="tablist" class="mb-5 tabs tabs-bordered tabs-lg">
    {#each routes as route}
        <a href="/home/{route}" role="tab" class="tab text-sm lg:text-lg {(data.path == route) && 'tab-active'}">
            { route.charAt(0).toUpperCase() + route.substring(1) }
        </a>
    {/each}
</div>

<slot />
