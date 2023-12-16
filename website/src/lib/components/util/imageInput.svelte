<script lang="ts">

    export let files: FileList;

    let src: string;

    $: {
        if (files && files.length) {
            const file = files[0];
            src = URL.createObjectURL(file);
        }
    }

</script>

{#if src}

    <div class="w-1/5">

        <img {src} alt="Post Image">

    </div>

{:else}

    <input bind:files={files} type="file" class="file-input file-input-bordered w-full max-w-xs" />

{/if}
