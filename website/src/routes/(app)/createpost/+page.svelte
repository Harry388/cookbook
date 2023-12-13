<script lang="ts">

    import { post } from '$lib/apiFetch';
    import { goto } from '$app/navigation';

    export let data;

    let title = '';
    let content = '';
    let files: FileList;

    async function createPost() {
        const formData = new FormData();
        const postStr = JSON.stringify({ title, content });
        formData.append('post', postStr);
        if (files) {
            for (const file of files) {
                formData.append('media', file);
            }
        }
        console.log(formData);
        await post('post', formData, {
            headers: {
                'Content-Type':  'remove'
            }
        }).run();
        goto('/user');
    }

</script>

<a class="btn btn-outline" href={`/user/${data.id}`}>Back</a>

<h3 class="font-bold text-lg">Create a Post</h3>
<div class="py-5">
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
        <input bind:files={files} type="file" multiple class="file-input file-input-bordered w-full max-w-xs" />
        <button class="btn btn-primary w-fit mt-5" on:click={createPost}>Create</button>
    </div>
</div>