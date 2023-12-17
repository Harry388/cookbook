<script lang="ts">

    export let files: FileList;
    export let multiple = false;

	let images: string[] = [];

    function onChange() {
        images = Array(files.length);
        for (const i in files) {
            const file = files[i];
            const reader = new FileReader();
            reader.addEventListener("load", () => {
                images[i] = String(reader.result);
            });
            reader.readAsDataURL(file);
        }
    }

</script>

<div class="flex">
    {#each images as image}
        <div class="w-1/5">
            <img src={image} alt="Post Image">
        </div>
    {/each}
</div>

<input bind:files={files} on:change={onChange} {multiple} type="file" accept="image/*" class="file-input file-input-bordered w-full max-w-xs" />

