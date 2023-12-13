<script lang="ts">

    import Image from '$lib/components/util/image.svelte';

    export let user: {
        id: number,
        display_name: string
    };

</script>

<div class="avatar placeholder">
    <div class="bg-neutral text-neutral-content rounded-full w-24">
        <Image src={`user/${user.id}/pfp`} alt={`${user.display_name}'s profile picture`}>
            <span class="text-3xl">{ user.display_name[0] }</span>
        </Image>
    </div>
</div> 