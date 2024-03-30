<script lang="ts">

    import { createPost } from '$lib/app/post';
    import CreatePost from '$lib/components/post/createPost.svelte';
    import AttachRecipe from '$lib/components/recipe/attachRecipe.svelte';
    import type { Tag } from '$lib/app/tag';
    import type { Recipe } from '$lib/app/recipe';


    export let data;

    let title = '';
    let content = '';
    let files: File[] = [];
    let community = data.community || -1;
    let tags: Tag[] = [];
    let recipes: Recipe[] = [];
    let step = 1;

    let maxSteps = 2;
    
    async function create() {
        if (!title) return;
        const c = community == -1 ? null : community;
        const t = tags.map(t => t.tag);
        const r = recipes.map(r => r.id);
        const response = await createPost(title, content, c, files, t, r).run();
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
    {#if step > 1}
        <button class="btn btn-warning w-fit mt-5" on:click={() => step--}>Back</button>
    {/if}
    {#if step < maxSteps}
        <button class="btn btn-warning w-fit mt-5" on:click={() => step++}>Next</button>
    {:else}
        <button class="btn btn-primary w-fit mt-5" on:click={create}>Create</button>
    {/if}
</div>
