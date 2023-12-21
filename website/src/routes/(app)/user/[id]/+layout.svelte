<script lang="ts">

    import Info from '$lib/components/user/info.svelte';
    import { setContext } from 'svelte';
    import { writable } from 'svelte/store';
    import type { User } from '$lib/app/user';

    export let data;

    const routes = ['posts', 'recipes'];

    const user = writable<User>();
    $: user.set(data.user);
    setContext('user', user);

</script>

<Info/>

<div role="tablist" class="my-5 tabs tabs-bordered tabs-lg">
    {#each routes as route}
        <a href={`/user/${$user.id}/${route}`} role="tab" class={`tab ${(data.path == route) && 'tab-active'}`}>
            { route.charAt(0).toUpperCase() + route.substring(1) }
        </a>
    {/each}
</div>

<slot/>