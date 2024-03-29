<script lang="ts">

    import { createPost } from '$lib/app/post';
    import CreatePost from '$lib/components/post/createPost.svelte';
    import AttachRecipe from '$lib/components/recipe/attachRecipe.svelte';
    import type { Tag } from '$lib/app/tag';
    import type { Recipe } from '$lib/app/recipe';
    import type { Snapshot } from './$types.js';

    type State = {
        title: string,
        content: string,
        files: File[],
        community: number,
        tags: Tag[],
        recipes: Recipe[],
        step: number
    };

    export let data;

    let state: State = {
        title: '',
        content: '',
        files: [],
        community: data.community || -1,
        tags: [],
        recipes: [],
        step: 1
    }

    let maxSteps = 2;

    async function create() {
        if (!state.title) return;
        const c = state.community == -1 ? null : state.community;
        const t = state.tags.map(t => t.tag);
        const response = await createPost(state.title, state.content, c, state.files, t).run();
        if (response.ok) {
            history.back();
        }
    }

    export const snapshot: Snapshot<State> = {
        capture: () => state,
        restore: value => (state = value)
    };

</script>

<div class="lg:w-1/2 m-auto">
    {#if state.step == 1}
        <h3 class="font-bold text-lg py-5">Create a Post</h3>
        <CreatePost bind:title={state.title} bind:content={state.content} bind:files={state.files} bind:community={state.community} bind:communities={data.communities} bind:tags={state.tags} />
    {:else if state.step == 2}
        <h3 class="font-bold text-lg py-5">Attach Recipes</h3>
        <AttachRecipe bind:recipes={state.recipes} options={data.userRecipes} create />
    {/if}
    {#if state.step > 1}
        <button class="btn btn-warning w-fit mt-5" on:click={() => state.step--}>Back</button>
    {/if}
    {#if state.step < maxSteps}
        <button class="btn btn-warning w-fit mt-5" on:click={() => state.step++}>Next</button>
    {:else}
        <button class="btn btn-primary w-fit mt-5" on:click={create}>Create</button>
    {/if}
</div>
