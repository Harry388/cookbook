<script lang="ts">

    export let data;

    function deleteAccount() {
        if (confirm('Are you sure?')) {
            
        }
    }

</script>

<h3 class="font-bold text-lg">Settings</h3>
<div class="py-5">
    <div class="form-control">
        <button class="btn btn-error w-fit mt-5" on:click={deleteAccount}>Delete Account</button>
    </div>
</div>