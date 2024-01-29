<script lang="ts">

    import { logout } from '$lib/auth/auth';
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { writable } from 'svelte/store';
    import { setContext } from 'svelte';

    export let data;

    const id = writable<number>();
    $: id.set(data.id);
    setContext('id', id);

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
                {#if data.showBack}
                    <button class="btn btn-outline" on:click={() => history.back()}>Back</button>
                {/if}
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
            <a href="/">Home</a>
            <a href="/user/{data.id}">Profile</a>
            <a href="/settings">Settings</a>
        </div>
    </div> 
    <div class="drawer-side">
        <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label> 
        <ul class="menu flex p-4 w-40 min-h-full bg-base-300 text-base-content">
            <!-- Sidebar content here -->
            <li><a href="/">Home</a></li>
            <li><a href="/community">Communities</a></li>
            <li><a href="/user/{data.id}">Profile</a></li>
            <li class="flex-grow bg-base-300"></li>
            <li><a href="/settings">Settings</a></li>
            <li class="place-self-start"><button class="btn btn-ghost" on:click={onLogOut}>Log Out</button></li>
    </div>
</div>
