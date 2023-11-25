<script>

    import { logout } from '$lib/auth/auth';
    import { goto } from '$app/navigation';

    export let data;

    async function onLogOut() {
        const response = await logout();
        if (response.ok) {
            goto('/login');
        }
    }

</script>

<div class="drawer lg:drawer-open">
    <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content flex flex-col items-center">
        <!-- Page content here -->
        <div class="navbar bg-base-100">
            <div class="navbar-start">
                <label for="my-drawer-2" class="btn btn-primary drawer-button lg:hidden">Open drawer</label>
            </div>
            <div class="navbar-center">
                <h1 class="font-bold text-3xl">CookBook</h1>
            </div>
            <div class="navbar-end">

            </div>
        </div>
        <slot />
    </div> 
    <div class="drawer-side">
        <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label> 
        <ul class="menu flex p-4 w-40 min-h-full bg-base-200 text-base-content">
            <!-- Sidebar content here -->
            <li><a href="/">Home</a></li>
            <li><a href="/user/{data.id}">Profile</a></li>
            <div class="flex-grow"></div>
            <li class="place-self-start"><button class="btn btn-ghost" on:click={onLogOut}>Log Out</button></li>
        </ul>
    </div>
</div>