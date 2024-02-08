import { get, post, remove, put } from '$lib/apiFetch';
import { getUser } from '$lib/app/user';
import type { FetchFn } from '$lib/apiFetch';
import type { PostFull, Post } from '$lib/app/post';

export type Community = {
    id: number,
    title: string,
    description: string | null,
    created: string,
    users: number,
    is_member: number,
    is_admin: number
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

export async function getCommunityPosts(id: number | string, fetch?: FetchFn): Promise<PostFull[]> {
    const response = await get(`community/${id}/post`).run(fetch);
    const posts: Post[] = await response.json();
    const postsFull: PostFull[] = [];
    for (const post of posts) {
        const [community, user] = await Promise.all([post.community_id ? getCommunity(post.community_id, fetch) : null, getUser(post.user_id, fetch)]);
        postsFull.push({
            ...post,
            community,
            user
        });
    }
    return postsFull;
}

export async function leaveCommunity(id: number | string, userId: number | string, fetch?: FetchFn): Promise<Response> {
    return await remove(`community/${id}/leave/${userId}`).run(fetch);
}

export async function joinCommunity(id: number | string, fetch?: FetchFn): Promise<Response> {
    return await post(`community/${id}/join`).run(fetch);
}

export async function updateCommunity(id: number | string, title: string | null, description: string | null, fetch?: FetchFn): Promise<Response> {
    return await put(`community/${id}`, { title, description }).run(fetch);
}

export async function deleteCommunity(id: number | string, fetch?: FetchFn): Promise<Response> {
    return await remove(`community/${id}`).run(fetch);
}

export async function createCommunity(title: string, desciption: string | null, fetch?: FetchFn): Promise<Response> {
    return await post(`community`, { title, desciption }).run(fetch);
}

