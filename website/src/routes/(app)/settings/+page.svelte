<script lang="ts">

    import { deleteUser } from '$lib/app/user';
    import { goto } from '$app/navigation';

    export let data;

    async function deleteAccount() {
        if (confirm('Are you sure?')) {
            const response = await deleteUser(data.id);
            if (response.ok) {
                goto('/login');
            }
        }
    }

</script>

<h3 class="font-bold text-lg">Settings</h3>
<div class="py-5">
    <div class="form-control">
        <button class="btn btn-error w-fit mt-5" on:click={deleteAccount}>Delete Account</button>
    </div>
</div>