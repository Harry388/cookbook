<script>

    import { goto } from '$app/navigation';
    import { login } from '$lib/auth/auth';

    export let data;

    let email = '';
    let password = '';
    let showPassword = false;

    let toCreateAccount = data.redirect ? `/createaccount?redirect=${data.redirect}` : '/createaccount';

    async function onLogIn() {
        const response = await login(email, password);
        if (response.ok) {
            goto(data.redirect ?? '/');
        }
    }

</script>


<h1 class="text-2xl font-bold">Log In</h1>
<form on:submit|preventDefault={onLogIn}>
    <div class="form-control">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">Email</span>
        </label>
        <input type="email" min="1" bind:value={email} placeholder="Email" class="input input-bordered" />
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">Password</span>
        </label>
        {#if showPassword}
        <input type="text" min="1" bind:value={password} placeholder="Password" class="input input-bordered" />
        {:else}
        <input type="password" min="1" bind:value={password} placeholder="Password" class="input input-bordered" />
        {/if}
        <label class="label cursor-pointer">
            <span class="label-text">Show Password</span> 
            <input type="checkbox" bind:checked={showPassword} class="checkbox checkbox-primary" />
        </label>
        <div class="label-text">Don't have an account? <a href={toCreateAccount}>Create Account</a></div>
    </div>
    <div class="form-control mt-6">
        <input type="submit" value="Log In" class="btn btn-primary" />
    </div>
</form>