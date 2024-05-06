<script lang="ts">

    import Image from '$lib/components/util/image.svelte';

    export let user: {
        id: number,
        display_name: string
    };

    export let small = false;

</script>

<div class="avatar placeholder">
    <div class="bg-neutral text-neutral-content rounded-full {small ? 'w-12' : 'w-24'}">
        <Image src="user/{user.id}/pfp" alt="{user.display_name}'s profile picture">
            <span class={small ? '' : 'text-3xl'}>{ user.display_name[0] }</span>
        </Image>
    </div>
</div> 
