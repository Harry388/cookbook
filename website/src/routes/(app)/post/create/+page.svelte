<script lang="ts">

    import { createPost } from '$lib/app/post';
    import ImageInput from '$lib/components/util/imageInput.svelte';

    let title = '';
    let content = '';
    let files: File[];

    async function create() {
        if (!title) return;
        const response = await createPost(title, content, files);
        if (response.ok) {
            history.back();
        }
    }

</script>

<h3 class="font-bold text-lg py-5">Create a Post</h3>
<div class="form-control">
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Title</span>
    </label>
    <input type="text" min="1" bind:value={title} placeholder="Title" class="input input-bordered" />
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Content</span>
    </label>
    <textarea class="textarea textarea-bordered" placeholder="Content" bind:value={content}></textarea>
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Media</span>
    </label>
    <ImageInput bind:files={files} multiple />
    <button class="btn btn-primary w-fit mt-5" on:click={create}>Create</button>
</div>