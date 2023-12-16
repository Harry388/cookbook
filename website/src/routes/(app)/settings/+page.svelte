<script lang="ts">

    import { deleteUser } from '$lib/app/user';
    import { logout } from '$lib/auth/auth';
    import { goto } from '$app/navigation';

    export let data;

    async function onLogOut() {
        const response = await logout().run();
        if (response.ok) {
            goto('/login');
        }
    }

    async function deleteAccount() {
        if (confirm('Are you sure?')) {
            const response = await deleteUser(data.id);
            if (response.ok) {
                await logout().run();
                goto('/login');
            }
        }
    }

</script>

<h3 class="font-bold text-lg">Settings</h3>
<div class="py-5">
    <div class="form-control">
        <button class="btn btn-primary w-fit" on:click={onLogOut}>Log Out</button>
        <button class="btn btn-error w-fit mt-5" on:click={deleteAccount}>Delete Account</button>
    </div>
</div>