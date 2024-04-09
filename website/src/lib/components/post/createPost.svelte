<script lang="ts">

    import ImageInput from '$lib/components/util/imageInput.svelte';
    import TagInput from '$lib/components/tag/tagInput.svelte';
    import Input from '$lib/components/util/input.svelte';
    import SelectInput from '$lib/components/util/selectInput.svelte';
    import type { Tag } from '$lib/app/tag';
    import type { Option } from '$lib/components/util/selectInput.svelte';

    export let title: string;
    export let content: string;
    export let files: File[];
    export let community: number;
    export let communities: Option[];
    export let tags: Tag[];

</script>

<div class="form-control">
    <SelectInput bind:value={community} options={communities} title="Pick Community" />
    <Input bind:value={title} title="Title" required />
    <Input bind:value={content} title="Content" long />
    <TagInput bind:tags={tags} />
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Media</span>
    </label>
    <ImageInput bind:files={files} multiple />
</div>
