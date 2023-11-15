<template>

    <div class="navbar bg-base-100">
        <div class="navbar-start">
            <label for="my-drawer-2" class="btn btn-primary drawer-button lg:hidden">Open drawer</label>
            <div>Logo</div>
        </div>
        <div class="navbar-center">
            <a class="btn btn-ghost text-3xl">Cook Book</a>
        </div>
        <div class="navbar-end"></div>
    </div>

    <div class="drawer lg:drawer-open">
        <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
        <div class="drawer-content flex flex-col items-center justify-center">
            <!-- Page content here -->
            <slot />
        </div> 
        <div class="drawer-side">
            <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label> 
            <ul class="menu p-4 w-80 min-h-full bg-base-300 text-base-content">
                <!-- Sidebar content here -->
                <li><NuxtLink to="/">Home</NuxtLink></li>
                <li><NuxtLink to="/communities">Communities</NuxtLink></li>
                <li><NuxtLink to="/search">Search</NuxtLink></li>
                <li><NuxtLink to="/messaging">Messaging</NuxtLink></li>
                <li><NuxtLink to="/profile">Profile</NuxtLink></li>
                <div class="dropdown">
                    <label tabindex="0" class="btn m-1">More</label>
                    <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
                        <li><a>Settings</a></li>
                        <li><a>Log Out</a></li>
                    </ul>
                </div>
            </ul>
        </div>
    </div>

</template>