<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import Share from '$lib/components/util/share.svelte';
    import Save from '$lib/components/entries/save.svelte';
    import { likePost, unlikePost } from '$lib/app/post';
    import { likeRecipe, unlikeRecipe } from '$lib/app/recipe';
    import { invalidateAll } from '$app/navigation';

    export let entry: {
        id: number,
        title: string,
        content?: string,
        description?: string,
        user_id: number,
        user_display_name: string,
        community_id?: number | null,
        community_title?: string,
        created: string,
        is_liked: number
    };
    export let link = false;
    export let type: 'post' | 'recipe';

    $: created = new Date(entry.created).toDateString();

    async function toggleLike() {
        if (entry.is_liked) {
            if (type == 'post') {
                await unlikePost(entry.id).run();
            }
            else {
                await unlikeRecipe(entry.id).run();
            }
        }
        else {
            if (type == 'post') {
                await likePost(entry.id).run();
            }
            else {
                await likeRecipe(entry.id).run();
            }
        }
        invalidateAll();
    }

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
            <button class="fa-{entry.is_liked ? 'solid' : 'regular'} fa-heart text-2xl" on:click={toggleLike}></button>
            <Save entryId={entry.id} {type} />
            <Share path="/{type}/{entry.id}" />
            {#if link }
                <a href="/{type}/{entry.id}" class="btn">More</a>
            {/if}
        </div>
    </div>
</div>
