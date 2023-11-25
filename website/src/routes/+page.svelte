<script>

    import { logout } from '$lib/auth/auth';
    import { goto } from '$app/navigation';

    async function onLogOut() {
        const response = await logout();
        if (response.ok) {
            goto('/login');
        }
    }

</script>

<div class="hero min-h-screen bg-base-200">
    <div class="hero-content flex-col lg:flex-row-reverse">
        <div class="text-center lg:text-left">
            <h1 class="text-5xl font-bold">Welcome to CookBook!</h1>
            <p class="py-6">Chomp chomp</p>
            <button class="btn btn-primary" on:click={onLogOut}>Log Out</button>
        </div>
    </div>
</div>