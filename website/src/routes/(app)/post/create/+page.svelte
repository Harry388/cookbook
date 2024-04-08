<script lang="ts">

    import { createPost } from '$lib/app/post';
    import CreatePost from '$lib/components/post/createPost.svelte';
    import AttachRecipe from '$lib/components/recipe/attachRecipe.svelte';
    import type { Tag } from '$lib/app/tag';
    import type { Recipe } from '$lib/app/recipe';
    import Post from '$lib/components/post/post.svelte';


    export let data;

    let title = '';
    let content = '';
    let files: File[] = [];
    let community = data.community || -1;
    let tags: Tag[] = [];
    let recipes: Recipe[] = [];
    let step = 1;

    let maxSteps = 2;

    let creating = false;
    
    async function create() {
        if (!title || creating) return;
        const c = community == -1 ? null : community;
        const t = tags.map(t => t.tag);
        const r = recipes.map(r => r.id);
        creating = true;
        const response = await createPost(title, content, c, files, t, r).run();
        creating = false;
        if (response.ok) {
            history.back();
        }
    }

</script>

<div class="lg:w-1/2 m-auto">
    {#if step == 1}
        <h3 class="font-bold text-lg py-5">Create a Post</h3>
        <CreatePost bind:title={title} bind:content={content} bind:files={files} bind:community={community} bind:communities={data.communities} bind:tags={tags} />
    {:else if step == 2}
        <h3 class="font-bold text-lg py-5">Attach Recipes</h3>
        <AttachRecipe bind:recipes={recipes} create />
    {/if}
    <div class="flex mt-5">
        {#if step > 1}
            <button class="text-lg fa-solid fa-arrow-left-long" on:click={() => step--}></button>
        {/if}
        <div class="flex-grow"></div>
        {#if step < maxSteps}
            <button class="text-lg fa-solid fa-arrow-right-long" on:click={() => step++}></button>
        {:else}
            <button class="btn btn-success btn-outline w-fit mt-5" on:click={create}>
                {#if creating}
                    <span class="loading loading-spinner"></span>
                {:else}
                    Create Post
                {/if}
            </button>
        {/if}
    </div>
</div>
