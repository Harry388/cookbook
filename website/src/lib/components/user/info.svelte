<script lang="ts">

    import type { User } from '$lib/app/user';

    export let user: User;

</script>

<div class="card w-full bg-base-100 shadow-xl">
    <div class="card-body">
        <div class="flex flex-col lg:flex-row gap-y-2">
            <div class="flex flex-1 gap-x-5 mb-1">
                <div class="avatar placeholder">
                    <div class="bg-neutral text-neutral-content rounded-full w-24">
                        <span class="text-3xl">{ user.display_name[0] }</span>
                    </div>
                </div> 
                <div class="flex flex-col gap-y-1">
                    <h2 class="card-title text-3xl">{ user.display_name }</h2>
                    <h2 class="card-title">@{ user.username }</h2>
                </div>
            </div>
            <p class="flex-1 font-semibold text-xl">{ user.followers } Followers</p>
            <p class="flex-1 font-semibold text-xl">{ user.following } Following</p>
        </div>
        {#if user.bio }
            <p>{ user.bio }</p>
        {/if}
    </div>
</div>