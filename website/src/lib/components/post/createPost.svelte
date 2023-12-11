<script lang="ts">

    import Modal from '$lib/components/util/modal.svelte';
    import { post } from '$lib/apiFetch';

    let title = '';
    let content = '';
    let files: FileList;

    function createPost() {
        const formData = new FormData();
        const postStr = JSON.stringify({ title, content });
        formData.append('post', postStr);
        if (files) {
            for (const file of files) {
                formData.append('media', file);
            }
        }
        console.log(formData);
        post('post', formData, {
            headers: {
                'Content-Type':  'remove'
            }
        }).run();
    }

</script>

<Modal>

    <button class="btn btn-outline" slot="button">Create Post</button>

    <svelte:fragment slot="modal">
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
            </div>
        </div>
    </svelte:fragment>

    <svelte:fragment slot="close">
        <button class="btn">Close</button>
        <button class="btn btn-primary" on:click={createPost}>Create</button>
    </svelte:fragment>

</Modal>