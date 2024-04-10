import { get, remove, post, put } from '$lib/apiFetch';
import type { FetchFn } from '$lib/apiFetch';

export type Follow = {
    id: number,
    username: string,
    display_name: string
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

export function getRequests() {
    return get<Follow[]>(`user/requests`);
}

export function removeFollower(userId: number | string, followingId: number | string) {
    return remove(`user/${userId}/unfollow/${followingId}`);
}

export function followUser(userId: number | string, followingId: number | string) {
    return  post(`user/${userId}/follow/${followingId}`);
}

export function acceptFollow(userId: number | string) {
    return put(`user/acceptfollow/${userId}`);
}
