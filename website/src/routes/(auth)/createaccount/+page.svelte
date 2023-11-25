<script>

    import { goto } from '$app/navigation';
    import { create, login } from '$lib/auth/auth';

    export let data;

    let username = '';
    let displayName = '';
    let email = '';
    let password = '';
    let showPassword = false;

    let toLogIn = data.redirect ? `/login?redirect=${data.redirect}` : '/login';

    async function onCreate() {
        const createResponse = await create(username, displayName, email, password);
        if (createResponse.ok) {
            const loginResponse = await login(email, password);
            if (loginResponse.ok) {
                goto(data.redirect ?? '/');
            }
        }
    }

</script>


<h1 class="text-2xl font-bold">Create Account</h1>
<form on:submit|preventDefault={onCreate}>
    <div class="form-control">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">Username</span>
        </label>
        <input type="text" min="1" bind:value={username} placeholder="Username" class="input input-bordered" />
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">Display Name</span>
        </label>
        <input type="text" min="1" bind:value={displayName} placeholder="Display Name" class="input input-bordered" />
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">Email</span>
        </label>
        <input type="text" min="1" bind:value={email} placeholder="Email" class="input input-bordered" />
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
        <div class="label-text">Already have an account? <a href={toLogIn}>Log In</a></div>
    </div>
    <div class="form-control mt-6">
        <input type="submit" value="Create" class="btn btn-primary" />
    </div>
</form>
  