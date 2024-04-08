<script lang="ts">

    import Share from '$lib/components/util/share.svelte';
    import Confirm from '$lib/components/util/confirm.svelte';
    import { getContext } from 'svelte';
    import { deleteCookbook } from '$lib/app/cookbook';
    import { invalidate } from '$app/navigation';
    import type { Cookbook } from '$lib/app/cookbook';

    export let cookbook: Cookbook;

    const id: number = getContext('id');

    let downloadLoading = false;
    let anchor: HTMLAnchorElement;

    async function remove() {
        const response = await deleteCookbook(cookbook.id).run();
        if (response.ok) {
            invalidate('app:cookbooks');
        }
    }

    async function download() {
        downloadLoading = true;
        const response = await fetch(`/cookbook/${cookbook.id}`);
        if (response.ok) {
            anchor.href = URL.createObjectURL(await response.blob());
            anchor.download = `${cookbook.title}.pdf`;
            anchor.click();
        }
        downloadLoading = false;
    }

</script>

<!-- svelte-ignore a11y-missing-attribute a11y-missing-content -->
<a bind:this={anchor} class="hidden"></a>

<div class="flex-grow card card-compact bg-base-100 shadow-xl">
    <div class="card-body">
        <div class="flex gap-x-5 items-start">
            {#if id == cookbook.user_id }
                <div class="flex-grow"></div>
                <Confirm let:show on:confirm={remove} id={cookbook.id}>
                    <div class="dropdown dropdown-end">
                        <div tabindex="0" role="button" class="pb-5 pr-5 m-1 fa-solid fa-ellipsis-vertical"></div>
                        <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
                            <li><a href="/cookbook/{cookbook.id}/edit">Edit</a></li>
                            <li><button on:click={show}>Delete</button></li>
                        </ul>
                    </div>
                </Confirm>
            {/if}
        </div>
        <h1 class="text-3xl card-title mt-2">{ cookbook.title }</h1>
    </div>
    <slot name="media" />
    <div class="card-body !pt-0">
        <p class="text-lg">{ cookbook.description }</p>
        <slot />
        <div class="flex justify-end gap-x-5 items-center">
            <Share path="/cookbook/{cookbook.id}" />
            {#if downloadLoading}
                <span class="loading loading-spinner"></span>
            {:else}
                <button class="fa-solid fa-download text-2xl" on:click={download}></button>
            {/if}
            <a target="_blank" href="/cookbook/{cookbook.id}" class="btn">Open</a>
        </div>
    </div>
</div>
