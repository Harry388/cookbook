<script lang="ts">

    import Info from '$lib/components/user/info.svelte';

    export let data;

    const routes = ['posts', 'recipes', 'albums'];

</script>

<Info user={data.user} id={data.id} />

{#if data.self || data.user.public || data.user.is_following}
    <div role="tablist" class="my-5 tabs tabs-bordered tabs-lg">
        {#each routes as route}
            <a href="/user/{data.user.id}/{route}" role="tab" class="tab {(data.path == route) && 'tab-active'}">
                { route.charAt(0).toUpperCase() + route.substring(1) }
            </a>
        {/each}
    </div>
{:else}
    User is private
{/if}

<slot/>
