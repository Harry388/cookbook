import { get, post, remove, put } from '$lib/apiFetch';
import type { FetchFn } from '$lib/apiFetch';
import type { Post } from '$lib/app/post';

export type Community = {
    id: number,
    title: string,
    description: string | null,
    created: string,
    users: number
};

export async function getCommunity(id: number | string, fetch?: FetchFn): Promise<Community> {
    const response = await get(`community/${id}`).run(fetch);
    const community: Community = await response.json();
    return community;
}

export async function getUserCommunities(userId: number | string, fetch?: FetchFn): Promise<Community[]> {
    const response = await get(`community/user/${userId}`).run(fetch);
    const communities: Community[] = await response.json();
    return communities;
}

export async function getCommunityPosts(id: number | string, fetch?: FetchFn): Promise<Post[]> {
    const response = await get(`community/${id}/post`).run(fetch);
    const posts: Post[] = await response.json();
    return posts;
}

