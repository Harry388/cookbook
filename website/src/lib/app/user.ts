import { get, put } from '$lib/apiFetch';
import type { FetchFn } from '$lib/apiFetch';

export type User = {
    id: number,
    username: string,
    display_name: string,
    bio: string | null,
    pfp: string | null,
    public: number,
    following: number,
    followers: number,
    is_following: number
}

export async function getUser(id: number | string, fetch?: FetchFn): Promise<User> {
    const userResponse = await get(`user/${id}`).run(fetch);
    const user: User = await userResponse.json();
    return user;
}

export async function updateUser(id: number | string, displayName: string, bio: string | null, files: FileList, fetch?: FetchFn): Promise<Response> {
    if (files) {
        const formData = new FormData();
        const file = files[0];
        formData.append('pic', file);
        await put(`user/${id}/pfp`, formData, {
            headers: {
                'Content-Type': 'remove'
            }
        }).run(fetch);
    }
    return await put(`user/${id}`, {
        display_name: displayName,
        bio
    }).run(fetch);
}