<script lang="ts">

    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let id;

    function show() {
        //@ts-ignore
        document.getElementById(`${id}confirmModal`).showModal();
    }

    function confirm() {
        dispatch('confirm');
    }

</script>

<slot {show} />
<dialog id="{id}confirmModal" class="modal modal-bottom sm:modal-middle">
    <div class="modal-box">
        <h3 class="font-bold text-lg">Are you sure?</h3>
        <div class="modal-action">
            <form method="dialog">
                <button class="btn btn-ghost mr-5">Cancel</button>
                <button class="btn btn-error" on:click={confirm}>Delete</button>
            </form>
        </div>
    </div>
    <form method="dialog" class="modal-backdrop">
        <button>close</button>
    </form>
</dialog>
