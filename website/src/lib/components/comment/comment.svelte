<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import { getContext } from 'svelte';
    import { deleteComment, updateComment } from '$lib/app/comment';
    import { invalidate } from '$app/navigation';
    import type { Comment } from '$lib/app/comment';

    export let comment: Comment;
    export let type: 'POST' | 'RECIPE';

    let editing = false;
    let editingContent = comment.content;

    const id: number = getContext('id');

    $: date = new Date(comment.created).toDateString()
    $: depends = `app:${type.toLowerCase()}`;

    async function update() {
        if (editing && editingContent) {
            const response = await updateComment(comment.id, editingContent).run();
            if (response.ok) {
                await invalidate(depends);
                editing = false;
                editingContent = comment.content;
            }
        }
    }

    async function remove() {
        const response = await deleteComment(comment.id).run();
        if (response.ok) {
            invalidate(depends);
        }
    }

</script>

<div class="chat chat-start">
    <div class="chat-image avatar">
        <ProfilePic user={{ id: comment.user_id, display_name: comment.user_display_name }} />
    </div>
    <div class="chat-header">
        { comment.user_display_name }
        <time class="text-xs opacity-50">{ date }</time>
    </div>
    <div class="chat-bubble">
        {#if editing }
            <input type="text" bind:value={editingContent}>
        {:else}
            { comment.content }
        {/if}
    </div>
    {#if comment.user_id == id }
        <button class="btn btn-error" on:click={remove}>Delete</button>
        {#if editing }
            <button class="btn btn-warning" on:click={update}>Save</button>
        {:else}
            <button class="btn btn-warning" on:click={() => editing = true}>Edit</button>
        {/if}
    {/if}
</div>
