<script lang="ts">

    import { logout } from '$lib/auth/auth';
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { setContext } from 'svelte';

    export let data;

    setContext('id', data.id);

    $: pageName = $page.route.id?.split('/')[2];

    async function onLogOut() {
        const response = await logout().run();
        if (response.ok) {
            goto('/login');
        }
    }

</script>

<div class="drawer lg:drawer-open">
    <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content flex flex-col items-center bg-base-200 h-full">
        <!-- Page content here -->
        <div class="navbar pt-3">
            <div class="navbar-start">
                <button class="btn btn-outline" on:click={() => history.back()}>Back</button>
            </div>
            <div class="navbar-center">
                <h1 class="font-bold text-4xl">{ $page.data.title || 'CookBook' }</h1>
            </div>
            <div class="navbar-end">

            </div>
        </div>
        <div class="h-full w-full p-5">
            <slot />
        </div>
        <div class="h-16"></div>
        <div class="btm-nav lg:hidden bg-base-200">
            <a href="/" class={pageName == undefined ? 'active' : ''}>Home</a>
            <a href="/community" class={pageName == 'community' ? 'active' : ''}>Communities</a>
            <a href="/user/{data.id}" class={pageName == 'user' ? 'active' : ''}>Profile</a>
            <a href="/settings" class={pageName == 'settings' ? 'active' : ''}>Settings</a>
        </div>
    </div> 
    <div class="drawer-side">
        <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label> 
        <ul class="menu flex p-4 w-40 min-h-full bg-base-300 text-base-content">
            <!-- Sidebar content here -->
            <li><a href="/" class={pageName == undefined ? 'active' : ''}>Home</a></li>
            <li><a href="/community" class={pageName == 'community' ? 'active' : ''}>Communities</a></li>
            <li><a href="/user/{data.id}" class={pageName == 'user' ? 'active' : ''}>Profile</a></li>
            <li class="flex-grow bg-base-300"></li>
            <li><a href="/settings" class={pageName == 'settings' ? 'active' : ''}>Settings</a></li>
            <li class="place-self-start" ><button class="btn btn-ghost" on:click={onLogOut}>Log Out</button></li>
        </ul>
    </div>
</div>
