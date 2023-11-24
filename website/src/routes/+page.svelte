<script>

    let text = '';

    async function login() {
        await fetch("http://localhost:8000/api/auth/login", {
            method: "post",
            body: JSON.stringify({
                email: "harry@example.com",
                password: "password"
            }),
            headers: {
                'Accept': '*',
                'Content-Type': 'application/json'
            },
            credentials: 'include'
        });
    }

    async function logout() {
        await fetch("http://localhost:8000/api/auth/logout", { method: 'post', credentials: 'include' })
    }

    async function test() {
        const response = await fetch("http://localhost:8000/api/auth/test", { credentials: 'include'});
        text = await response.json();
    }

</script>

<div class="drawer lg:drawer-open">
    <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content flex flex-col items-center justify-center">
        <!-- Page content here -->
        <div class="navbar bg-base-100">
            <label for="my-drawer-2" class="btn btn-primary drawer-button lg:hidden">Open drawer</label>
            <button class="btn btn-ghost text-xl">CookBook</button>
        </div>
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="text-5xl font-bold">CookBook</h1>
                    <p class="py-6">Big munchies. Chomp chomp.</p>
                    <button on:click={login} class="btn btn-success">Log In</button>
                    <button on:click={logout} class="btn btn-warning">Log Out</button>
                    <button on:click={test} class="btn btn-primary">Test</button>
                    <p class="py-6">ID: { text }</p>
                    <button on:click={() => text = ''} class="btn btn-primary">Clear</button>
                </div>
            </div>
        </div>
    </div> 
    <div class="drawer-side">
        <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label> 
        <ul class="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
            <!-- Sidebar content here -->
            <li><a>Sidebar Item 1</a></li>
            <li><a>Sidebar Item 2</a></li>
        </ul>
    </div>
</div>