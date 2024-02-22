import { get, post, remove, put } from '$lib/apiFetch';
import type { Post } from '$lib/app/post';

export type Community = {
    id: number,
    title: string,
    description: string | null,
    created: string,
    users: number,
    is_member: number,
    is_admin: number,
    public: number
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

export function updateCommunity(id: number | string, title: string | null, description: string | null, isPublic: boolean) {
    return put(`community/${id}`, { title, description, public: isPublic });
}

export function deleteCommunity(id: number | string) {
    return remove(`community/${id}`);
}

export function createCommunity(title: string, description: string | null) {
    return post(`community`, { title, description });
}

export function removePost(id: number | string, post_id: number | string) {
    return remove(`community/${id}/removepost/${post_id}`);
}

export function acceptMember(id: number | string, userId: number | string) {
    return put(`community/${id}/acceptmember/${userId}`);
}
