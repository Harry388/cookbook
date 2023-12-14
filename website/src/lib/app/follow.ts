import { get, remove, post } from '$lib/apiFetch';
import type { FetchFn } from '$lib/apiFetch';

export type Follow = {
    id: number,
    username: string,
    display_name: string,
    pfp: string | null
}

export async function getUserFollow(userId: number | string, fetch?: FetchFn): Promise<{followers: Follow[], following: Follow[]}> {
    const [responseFollowers, responseFollowing] = await Promise.all([
        get(`user/${userId}/followers`).run(fetch),
        get(`user/${userId}/following`).run(fetch)
    ]);
    const [followers, following]: [Follow[], Follow[]] = await Promise.all([
        responseFollowers.json(),
        responseFollowing.json()
    ]);
    return { followers, following };
}

export async function removeFollower(userId: number | string, followingId: number | string, fetch?: FetchFn): Promise<Response> {
    return await remove(`user/${userId}/unfollow/${followingId}`).run(fetch);
}

export async function followUser(userId: number | string, followingId: number | string, fetch?: FetchFn): Promise<Response> {
    return await post(`user/${userId}/follow/${followingId}`).run(fetch);
}