<script lang="ts">

    import ProfilePic from '$lib/components/user/profilePic.svelte';
    import type { Comment } from '$lib/app/comment';

    export let comment: Comment;

    $: date = new Date(comment.created).toDateString()

</script>

<div class="chat chat-start">
    <div class="chat-image avatar">
        <ProfilePic user={{ id: comment.user_id, display_name: comment.user_display_name }} />
    </div>
    <div class="chat-header">
        { comment.user_display_name }
        <time class="text-xs opacity-50">{ date }</time>
    </div>
    <div class="chat-bubble">{ comment.content }</div>
</div>
