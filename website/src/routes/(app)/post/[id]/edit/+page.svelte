<script lang="ts">

    import { updatePost, addPostRecipe, deletePostRecipe } from '$lib/app/post';
    import { addEntryTags, removeEntryTags } from '$lib/app/tag';
    import { invalidate } from '$app/navigation';
    import Input from '$lib/components/util/input.svelte';
    import TagInput from '$lib/components/tag/tagInput.svelte';
    import AttachRecipe from '$lib/components/recipe/attachRecipe.svelte';
    import type { Tag } from '$lib/app/tag';

    export let data;

    let title = data.post.title;
    let content = data.post.content || '';

    async function save() {
        const response = await updatePost(data.post.id, title, content).run();
        if (response.ok) {
            invalidate('app:post');
        }
    }

    async function addRecipe(event: CustomEvent<number>) {
        const response = await addPostRecipe(data.post.id, event.detail).run();
        if (response.ok) {
            invalidate('app:post');
        }
    }

    async function deleteRecipe(event: CustomEvent<number>) {
        const response = await deletePostRecipe(data.post.id, event.detail).run();
        if (response.ok) {
            invalidate('app:post');
        }
    }

    async function addTag(event: CustomEvent<string>) {
        const response = await addEntryTags(data.post.id, [event.detail], 'post').run();
        if (response.ok) {
            invalidate('app:post');
        }
    }

    async function removeTag(event: CustomEvent<Tag>) {
        const response = await removeEntryTags(data.post.id, [event.detail.id], 'post').run();
        if (response.ok) {
            invalidate('app:post');
        }
    }

</script>

<div class="lg:w-1/2 m-auto">

    <h3 class="font-bold text-lg py-5">Edit Post</h3>
    <Input bind:value={title} title="Title" edit on:save={save} />
    <Input bind:value={content} title="Content" edit on:save={save} long />
    <TagInput tags={data.tags} edit on:add={addTag} on:remove={removeTag} />

    <h3 class="font-bold text-lg py-5">Attach Recipes</h3>
    <AttachRecipe recipes={data.recipes} edit on:add={addRecipe} on:remove={deleteRecipe} create />

</div>
