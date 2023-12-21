<button onclick="my_modal_2.showModal()"><slot name="button"/></button>
<dialog id="my_modal_2" class="modal modal-bottom sm:modal-middle">
    <div class="modal-box">
        <slot name="modal"/>
        <div class="modal-action">
            <form method="dialog">
                <slot name="close"/>
            </form>
        </div>
    </div>
    <form method="dialog" class="modal-backdrop">
        <button>close</button>
    </form>
</dialog>