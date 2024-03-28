<script lang="ts">

    import { createPost } from '$lib/app/post';
    import ImageInput from '$lib/components/util/imageInput.svelte';
    import TagInput from '$lib/components/tag/tagInput.svelte';
    import Input from '$lib/components/util/input.svelte';
    import SelectInput from '$lib/components/util/selectInput.svelte';

    export let data;

    let title = '';
    let content = '';
    let files: File[];
    let community: number = data.community || -1;
    let tags: string[];

    async function create() {
        if (!title) return;
        const c = community == -1 ? null : community;
        const response = await createPost(title, content, c, files, tags).run();
        if (response.ok) {
            history.back();
        }
    }

</script>

<h3 class="font-bold text-lg py-5">Create a Post</h3>
<div class="form-control">
    <SelectInput bind:value={community} options={data.communities} title="Pick Community" />
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
