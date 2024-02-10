import { get, put, remove } from '$lib/apiFetch';

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

export function getUser(id: number | string) {
    return get<User>(`user/${id}`);
}

export function updateUser(id: number | string, username: string, displayName: string, bio: string | null, pfp: File | null) {
    if (pfp) {
        const formData = new FormData();
        formData.append('pic', pfp);
        put(`user/${id}/pfp`, formData, {
            headers: {
                'Content-Type': 'remove'
            }
        });
    }
    return put(`user/${id}`, {
        username,
        display_name: displayName,
        bio
    });
}

export function deleteUser(id: number | string) {
    return remove(`user/${id}`);
}
