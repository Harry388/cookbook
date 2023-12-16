<script lang="ts">

    export let files: FileList;

	let image: string;

    function onChange() {
        for (const file of files) {
            const reader = new FileReader();
            reader.addEventListener("load", () => {
                image = String(reader.result);
            });
            reader.readAsDataURL(file);
        }
    }

</script>

{#if image}
    <div class="w-1/5">
        <img src={image} alt="Post Image">
    </div>
{/if}

<input bind:files={files} on:change={onChange} type="file" accept="image/*" class="file-input file-input-bordered w-full max-w-xs" />

