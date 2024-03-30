<script>

    import Input from '$lib/components/util/input.svelte';
    import { goto } from '$app/navigation';
    import { login } from '$lib/auth/auth';

    export let data;

    let email = '';
    let password = '';

    let toCreateAccount = data.redirect ? `/createaccount?redirect=${data.redirect}` : '/createaccount';

    async function onLogIn() {
        const response = await login(email, password).run();
        if (response.ok) {
            goto(data.redirect ?? '/');
        }
    }

</script>


<h1 class="text-2xl font-bold">Log In</h1>
<form on:submit|preventDefault={onLogIn}>
    <div class="form-control">
        <Input bind:value={email} type="email" title="Email" />
        <Input bind:value={password} type="password" title="Password" />
        <div class="label-text">Don't have an account? <a href={toCreateAccount}>Create Account</a></div>
    </div>
    <div class="form-control mt-6">
        <input type="submit" value="Log In" class="btn btn-primary" />
    </div>
</form>
