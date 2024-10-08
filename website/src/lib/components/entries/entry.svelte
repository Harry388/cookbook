<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import Share from '$lib/components/util/share.svelte';
    import Save from '$lib/components/entries/save.svelte';
    import Tags from '$lib/components/tag/tags.svelte';
    import Confirm from '$lib/components/util/confirm.svelte';
    import { deletePost, likePost, unlikePost } from '$lib/app/post';
    import { deleteRecipe, likeRecipe, unlikeRecipe } from '$lib/app/recipe';
    import { getEntryTags } from '$lib/app/tag';
    import { invalidateAll } from '$app/navigation';
    import { getContext, onMount } from 'svelte';
    import { page } from '$app/stores';
    import type { Tag } from '$lib/app/tag';

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
        is_liked: number,
        likes: number,
        comments: number,
        links: number
    };
    export let link = false;
    export let type: 'post' | 'recipe';

    $: created = new Date(entry.created).toDateString();

    const id: number = getContext('id');

    let tags: Tag[] = [];

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

    async function remove() {
        if (type == 'post') {
            await deletePost(entry.id).run();
        }
        else {
            await deleteRecipe(entry.id).run();
        }
        if ($page.url.pathname == `/${type}/${entry.id}`) {
            history.back();
        }
        await invalidateAll();
    }

    onMount(async () => {
        tags = await getEntryTags(entry.id, type).json();
    });

</script>

<div class="flex-grow card card-compact bg-base-100 shadow-xl">
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
            {#if id == entry.user_id }
                <Confirm let:show on:confirm={remove} id={entry.id}>
                    <div class="dropdown dropdown-end">
                        <div tabindex="0" role="button" class="pb-5 pr-5 m-1 fa-solid fa-ellipsis-vertical"></div>
                        <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
                        <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
                            <li><a href="/{type}/{entry.id}/edit">Edit</a></li>
                            <li><button on:click={show}>Delete</button></li>
                        </ul>
                    </div>
                </Confirm>
            {/if}
        </div>
        <h1 class="text-3xl card-title mt-2">{ entry.title }</h1>
        <Tags {tags} />
    </div>
    <slot name="media" />
    <div class="card-body !pt-0">
        <p class="text-lg">{ entry.content || entry.description || '' }</p>
        <slot />
        <div class="flex justify-end gap-x-5 items-center">
            <button class="fa-{entry.is_liked ? 'solid' : 'regular'} fa-heart text-2xl" on:click={toggleLike}></button>
            <div class="-ml-2 text-xl">{ entry.likes }</div>
            <svelte:element this={link ? 'a' : 'div'} href="/{type}/{entry.id}" class="flex justify-end gap-x-5 items-center">
                <i class="fa-regular fa-comment text-2xl"></i>
                <div class="-ml-2 text-xl">{ entry.comments }</div>
                <i class="fa-solid fa-{type == 'post' ? 'list-ul' : 'image'} text-2xl"></i>
                <div class="-ml-2 text-xl">{ entry.links }</div>
            </svelte:element>
            <Save entryId={entry.id} {type} />
            <Share path="/{type}/{entry.id}" />
        </div>
    </div>
</div>
