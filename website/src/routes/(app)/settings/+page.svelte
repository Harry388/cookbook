<script lang="ts">

    export let data;

    let username = data.user.username;

    function save() {

    }

    function deleteAccount() {

    }

</script>

<h3 class="font-bold text-lg">Settings</h3>
<div class="py-5">
    <div class="form-control">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">Username</span>
        </label>
        <input type="text" min="1" bind:value={username} placeholder="Display Name" class="input input-bordered" />
        <button class="btn btn-primary w-fit mt-5" on:click={save}>Save</button>
        <button class="btn btn-error w-fit mt-5" on:click={deleteAccount}>Delete Account</button>
    </div>
</div>