<script>

    export let data;

</script>

<div class="card w-full bg-base-100 shadow-xl">
    <div class="card-body">
        <div class="flex flex-col lg:flex-row gap-y-2">
            <div class="flex flex-1 gap-x-5 mb-1">
                <div class="avatar placeholder">
                    <div class="bg-neutral text-neutral-content rounded-full w-24">
                        <span class="text-3xl">{ data.user.display_name[0] }</span>
                    </div>
                </div> 
                <div class="flex flex-col gap-y-1">
                    <h2 class="card-title text-3xl">{ data.user.display_name }</h2>
                    <h2 class="card-title">@{ data.user.username }</h2>
                </div>
            </div>
            <p class="flex-1 font-semibold text-xl">0 Followers</p>
            <p class="flex-1 font-semibold text-xl">0 Following</p>
        </div>
        {#if data.user.bio }
            <p>{ data.user.bio }</p>
        {/if}
    </div>
</div>
