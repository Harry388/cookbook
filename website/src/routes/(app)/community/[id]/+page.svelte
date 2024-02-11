<script lang="ts">

    import Info from '$lib/components/community/info.svelte';
    import Posts from '$lib/components/post/posts.svelte';

    export let data;

</script>

<Info community={data.community} id={data.id} />

<div class="pt-5">

    <Posts posts={data.posts} />

</div>
