<script lang="ts">

    import { updatePost } from '$lib/app/post';
    import { goto } from '$app/navigation';

    export let data;

    let title = data.post.title;
    let content = data.post.content;

    async function save() {
        const response = await updatePost(data.post.id, title, content);
        if (response.ok) {
            goto(`/post/${data.post.id}`, {
                invalidateAll: true
            });
        }
    }

</script>

<a class="btn btn-outline" href="/post/{data.post.id}">Back</a>

<h3 class="font-bold text-lg py-5">Edit Post</h3>
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
    <!-- <label class="label">
        <span class="label-text">Media</span>
    </label>
    <ImageInput bind:files={files} multiple /> -->
    <button class="btn btn-primary w-fit mt-5" on:click={save}>Save</button>
</div>