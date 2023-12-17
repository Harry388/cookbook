import { get, put, remove } from '$lib/apiFetch';
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
    is_following: number,
    created: string
}

export async function getUser(id: number | string, fetch?: FetchFn): Promise<User> {
    const userResponse = await get(`user/${id}`).run(fetch);
    const user: User = await userResponse.json();
    return user;
}

export async function updateUser(id: number | string, username: string, displayName: string, bio: string | null, pfp: File | null, fetch?: FetchFn): Promise<Response> {
    if (pfp) {
        const formData = new FormData();
        formData.append('pic', pfp);
        await put(`user/${id}/pfp`, formData, {
            headers: {
                'Content-Type': 'remove'
            }
        }).run(fetch);
    }
    return await put(`user/${id}`, {
        username,
        display_name: displayName,
        bio
    }).run(fetch);
}

export async function deleteUser(id: number | string, fetch?: FetchFn): Promise<Response> {
    return await remove(`user/${id}`).run(fetch);
}