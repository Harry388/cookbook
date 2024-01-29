<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';
    import { setContext } from 'svelte';
    import { writable } from 'svelte/store';
    import type { Post } from '$lib/app/post';

    export let data;

    const posts = writable<Post[]>();
    $: posts.set(data.posts);
    setContext('posts', posts);

</script>

<div class="card w-full bg-base-100 shadow-xl mb-5">
    <div class="card-body">
        <div class="flex flex-col lg:flex-row gap-y-2 gap-x-5">
            <div class="flex-1 flex gap-x-5 mb-1">
                <!-- <ProfilePic user={$user}/> -->
                <div class="flex flex-col gap-y-1">
                    <h2 class="card-title text-3xl">{ data.community.title }</h2>
                </div>
            </div>
            <div class="flex-1">
                <div class="flex lg:gap-x-10 mb-5">
                    <div class="flex-2 lg:flex-1 font-semibold text-xl">{ data.community.users } Members</div>
                </div>
            </div>
            <div class="flex-1">
                <button class="btn btn-outline w-full">Join</button>
            </div>
        </div>
        {#if data.community.description }
            <p>{ data.community.description }</p>
        {/if}
    </div>
</div>

<Posts />
