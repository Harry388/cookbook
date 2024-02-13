<script lang="ts">

    import CommentComponent from '$lib/components/comment/comment.svelte';
    import type { Comment } from '$lib/app/comment';

    export let comments: Comment[]

</script>

{#each comments as comment}
   <CommentComponent {comment} /> 
{/each}
