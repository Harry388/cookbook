import { get, remove, post } from '$lib/apiFetch';
import type { FetchFn } from '$lib/apiFetch';

export type Follow = {
    id: number,
    username: string,
    display_name: string,
    pfp: string | null
}

export function getUserFollow(userId: number | string) {
    return {
        async json(fetch?: FetchFn) {
            const [followers, following] = await Promise.all([
                get<Follow[]>(`user/${userId}/followers`).json(fetch),
                get<Follow[]>(`user/${userId}/following`).json(fetch)
            ]);
            return { followers, following };
        }
    }
}

export function removeFollower(userId: number | string, followingId: number | string) {
    return remove(`user/${userId}/unfollow/${followingId}`);
}

export function followUser(userId: number | string, followingId: number | string) {
    return  post(`user/${userId}/follow/${followingId}`);
}
