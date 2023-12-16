<script lang="ts">

    import PostPreview from '$lib/components/post/postPreview.svelte';

    export let data;

</script>

<a class="btn btn-outline" href="/user/{data.post.user_id}">Back</a>

<div class="flex flex-col items-center lg:flex-row  mt-5">

    <PostPreview post={data.post} />

</div>

