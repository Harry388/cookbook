<script>

    import { goto } from '$app/navigation';
    import { create, login } from '$lib/auth/auth';
    import Input from '$lib/components/util/input.svelte';

    export let data;

    let username = '';
    let displayName = '';
    let email = '';
    let password = '';

    let toLogIn = data.redirect ? `/login?redirect=${data.redirect}` : '/login';

    async function onCreate() {
        const createResponse = await create(username, displayName, email, password).run();
        if (createResponse.ok) {
            const loginResponse = await login(email, password).run();
            if (loginResponse.ok) {
                goto(data.redirect ?? '/');
            }
        }
    }

</script>


<h1 class="text-2xl font-bold">Create Account</h1>
<form on:submit|preventDefault={onCreate}>
    <div class="form-control">
        <Input bind:value={username} title="Username" />
        <Input bind:value={displayName} title="Display Name" />
        <Input bind:value={email} title="Email" type="email" />
        <Input bind:value={password} title="Password" type="password" />
        <div class="label-text">Already have an account? <a href={toLogIn}>Log In</a></div>
    </div>
    <div class="form-control mt-6">
        <input type="submit" value="Create" class="btn btn-primary" />
    </div>
</form>
  
