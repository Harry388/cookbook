<script lang="ts">

    import { deleteUser, updateUser } from '$lib/app/user';
    import { logout } from '$lib/auth/auth';
    import { goto, invalidate } from '$app/navigation';
    import ImageInput from '$lib/components/util/imageInput.svelte';
    import Input from '$lib/components/util/input.svelte';

    export let data;

    let username = data.user.username;
    let displayName = data.user.display_name;
    let bio = data.user.bio || '';
    let isPublic = Boolean(data.user.public);
    let files: File[];

    async function savePfp() {
        const pfp = files.length ? files[0] : null;
        updateUser(data.id, username, displayName, bio, pfp, isPublic).run();
    }

    async function save() {
        const response = await updateUser(data.id, username, displayName, bio, null, isPublic).run();
        if (response.ok) {
            invalidate('app:settings');
        }
    }

    async function onLogOut() {
        const response = await logout().run();
        if (response.ok) {
            goto('/login');
        }
    }

    async function deleteAccount() {
        if (confirm('Are you sure?')) {
            const response = await deleteUser(data.id).run();
            if (response.ok) {
                await logout().run();
                goto('/login');
            }
        }
    }

</script>

<div class="lg:w-1/2 m-auto">
    <h3 class="font-bold text-lg py-5">Edit Profile</h3>
    <div class="form-control">
        <Input bind:value={username} title="User Name" edit on:save={save} />
        <Input bind:value={displayName} title="Display Name" edit on:save={save} />
        <Input bind:value={bio} title="Bio" edit on:save={save} long />
        <label class="label" for="#public">
            <span class="label-text">Public</span>
        </label>
        <input id="public" type="checkbox" class="checkbox checkbox-primary" bind:checked={isPublic} on:change={save} />
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">Profile Picture</span>
        </label>
        <ImageInput bind:files={files} />
        <button class="btn btn-primary w-fit mt-5" on:click={savePfp}>Update Profile Picture</button>
    </div>
    <h3 class="font-bold text-lg py-5">Other Settings</h3>
    <div class="form-control">
        <button class="btn btn-primary w-fit" on:click={onLogOut}>Log Out</button>
        <button class="btn btn-error w-fit mt-5" on:click={deleteAccount}>Delete Account</button>
    </div>
</div>
