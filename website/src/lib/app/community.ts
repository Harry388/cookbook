import { get, post, remove, put } from '$lib/apiFetch';
import type { Post } from '$lib/app/post';

export type Community = {
    id: number,
    title: string,
    description: string | null,
    created: string,
    users: number,
    is_member: number,
    is_admin: number
};

export function getCommunity(id: number | string) {
    return get<Community>(`community/${id}`);
}

export function getUserCommunities(userId: number | string) {
    return get<Community[]>(`community/user/${userId}`);
}

export function getCommunityPosts(id: number | string) {
    return get<Post[]>(`community/${id}/post`);
}

export function leaveCommunity(id: number | string, userId: number | string) {
    return remove(`community/${id}/leave/${userId}`);
}

export function joinCommunity(id: number | string) {
    return post(`community/${id}/join`);
}

export function updateCommunity(id: number | string, title: string | null, description: string | null) {
    return put(`community/${id}`, { title, description });
}

export function deleteCommunity(id: number | string) {
    return remove(`community/${id}`);
}

export function createCommunity(title: string, desciption: string | null) {
    return post(`community`, { title, desciption });
}

