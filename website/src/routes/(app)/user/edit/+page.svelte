<script lang="ts">

    import { updateUser } from '$lib/app/user';
    import { goto } from '$app/navigation';
    import ImageInput from '$lib/components/util/imageInput.svelte';

    export let data;

    let username = data.user.username;
    let displayName = data.user.display_name;
    let bio = data.user.bio;
    let files: File[];

    async function editProfile() {
        const pfp = files.length ? files[0] : null;
        const response = await updateUser(data.id, username, displayName, bio, pfp).run();
        if (response.ok) {
            goto('/user');
        }
    }

</script>

<h3 class="font-bold text-lg py-5">Edit Profile</h3>
<div class="form-control">
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">User Name</span>
    </label>
    <input type="text" min="1" bind:value={username} placeholder="User Name" class="input input-bordered" />
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Display Name</span>
    </label>
    <input type="text" min="1" bind:value={displayName} placeholder="Display Name" class="input input-bordered" />
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Bio</span>
    </label>
    <textarea class="textarea textarea-bordered" placeholder="Bio" bind:value={bio}></textarea>
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label class="label">
        <span class="label-text">Profile Picture</span>
    </label>
    <ImageInput bind:files={files} />
    <button class="btn btn-primary w-fit mt-5" on:click={editProfile}>Save</button>
</div>
