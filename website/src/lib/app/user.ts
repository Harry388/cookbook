import { get, put, remove } from '$lib/apiFetch';
import type { Entry } from '$lib/app/entry';

export type User = {
    id: number,
    username: string,
    display_name: string,
    bio: string | null,
    public: number,
    following: number,
    followers: number,
    is_following: number,
    is_requested: number,
    created: string
}

export function getUser(id: number | string) {
    return get<User>(`user/${id}`);
}

export function updateUser(id: number | string, username: string, displayName: string, bio: string | null, isPublic: boolean) {
    return put(`user/${id}`, { username, display_name: displayName, bio, public: isPublic });
}

export function setUserPfp(id: number | string, pfp: File) {
    const formData = new FormData();
    formData.append('pic', pfp);
    return put(`user/${id}/pfp`, formData, {
        headers: {
            'Content-Type': 'remove'
        }
    });
}

export function removeUserPfp(id: number | string) {
    return remove(`user/${id}/pfp`);
}

export function deleteUser(id: number | string) {
    return remove(`user/${id}`);
}

export function getUserFeed() {
    return get<Entry[]>('user/feed');
}
