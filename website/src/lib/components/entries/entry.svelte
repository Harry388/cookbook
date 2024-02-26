<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import Share from '$lib/components/util/share.svelte';
    import Save from '$lib/components/entries/save.svelte';

    export let entry: {
        id: number,
        title: string,
        content?: string,
        description?: string,
        user_id: number,
        user_display_name: string,
        community_id?: number | null,
        community_title?: string,
        created: string
    };
    export let link = false;
    export let type: 'post' | 'recipe';

    let liked = false;

    $: created = new Date(entry.created).toDateString();

</script>

<div class="card card-compact bg-base-100 shadow-xl">
    <div class="card-body">
        <div class="flex gap-x-5 items-start">
            <ProfilePic user={{ id: entry.user_id, display_name: entry.user_display_name }} small />
            <div>
                <a class="card-title w-fit" href="/user/{entry.user_id}">{ entry.user_display_name }</a>
                {#if entry.community_id != null }
                    <a class="w-fit" href="/community/{entry.community_id}">{ entry.community_title }</a>
                {/if}
            </div>
            <div class="flex-grow"></div>
            <div>{ created }</div>
        </div>
        <h1 class="text-3xl card-title mt-2">{ entry.title }</h1>
    </div>
    <slot name="media" />
    <div class="card-body !pt-0">
        <p class="text-lg">{ entry.content || entry.description || '' }</p>
        <slot />
        <div class="flex justify-end gap-x-5">
            <button class="fa-{liked ? 'solid' : 'regular'} fa-heart text-2xl" on:click={() => liked = !liked}></button>
            <Share path="/{type}/{entry.id}" />
            <Save entryId={entry.id} {type} />
            {#if link }
                <a href="/{type}/{entry.id}" class="btn">More</a>
            {/if}
        </div>
    </div>
</div>
