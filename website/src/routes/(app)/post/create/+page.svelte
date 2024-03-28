<script lang="ts">

    import { createPost } from '$lib/app/post';
    import ImageInput from '$lib/components/util/imageInput.svelte';
    import TagInput from '$lib/components/tag/tagInput.svelte';
    import Input from '$lib/components/util/input.svelte';

    export let data;

    let title = '';
    let content = '';
    let files: File[];
    let community: number | null = data.community;
    let tags: string[];

    async function create() {
        if (!title) return;
        const response = await createPost(title, content, community, files, tags).run();
        if (response.ok) {
            history.back();
        }
    }

</script>

<h3 class="font-bold text-lg py-5">Create a Post</h3>
<div class="form-control">
    <label class="label" for="#community">
        <span class="label-text">Community</span>
    </label>
    <select id="community" bind:value={community} class="select select-bordered">
        <option value={null} selected>Pick one</option>
        {#each data.communities as community}
            <option value={community.id}>{ community.title }</option>
        {/each}
    </select>
    <Input bind:value={title} title="Title" />
    <Input bind:value={content} title="Content" long />
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Tags</span>
    </label>
    <TagInput bind:tags={tags} />
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Media</span>
    </label>
    <ImageInput bind:files={files} multiple />
    <button class="btn btn-primary w-fit mt-5" on:click={create}>Create</button>
</div>
