<script lang="ts">

    import { updateUser } from '$lib/app/user';
    import { goto } from '$app/navigation';

    export let data;

    let displayName = data.user.display_name;
    let bio = data.user.bio;
    let files: FileList;

    async function editProfile() {
        await updateUser(data.id, displayName, bio, files);
        goto('user');
    }

</script>

<a class="btn btn-outline" href={`/user/${data.id}`}>Back</a>

<h3 class="font-bold text-lg">Edit Profile</h3>
<div class="py-5">
    <div class="form-control">
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
        <input bind:files={files} type="file" class="file-input file-input-bordered w-full max-w-xs" />
        <button class="btn btn-primary w-fit mt-5" on:click={editProfile}>Save</button>
    </div>
</div>