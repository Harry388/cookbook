<script lang="ts">

    import { goto } from '$app/navigation';
    import { login } from '$lib/auth/auth';
    import type { PageData } from './$types';

    export let data: PageData;

    let email = '';
    let password = '';
    let showPassword = false;

    async function onLogIn() {
        const response = await login(email, password);
        if (response.ok) {
            goto(data?.redirect ?? '/');
        }
    }

</script>

<div class="hero min-h-screen bg-base-200">
    <div class="hero-content flex-col lg:flex-row-reverse">
        <div class="text-center lg:text-left">
            <h1 class="text-5xl font-bold">Welcome to CookBook!</h1>
            <p class="py-6">Chomp chomp</p>
        </div>
        <div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
            <div class="card-body">
                <h1 class="text-2xl font-bold">Log In</h1>
                <form on:submit={onLogIn}>
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
                        <div class="form-control">
                            <label class="label cursor-pointer">
                                <span class="label-text">Show Password</span> 
                                <input type="checkbox" bind:checked={showPassword} class="checkbox checkbox-primary" />
                            </label>
                        </div>
                    </div>
                    <div class="form-control mt-6">
                        <input type="submit" value="Log In" class="btn btn-primary" />
                    </div>
                </form>
            </div>
        </div>
    </div>
</div>