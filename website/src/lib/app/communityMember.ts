import { get, put } from '$lib/apiFetch';

export type CommunityMember = {
    id: number,
    username: string,
    display_name: string,
    permission: 'ADMIN' | 'USER'
}

export function getCommunityMembers(id: number | string) {
    return get<CommunityMember[]>(`community/${id}/members`);
}

export function getCommunityRequests(id: number | string) {
    return get<CommunityMember[]>(`community/${id}/requests`);
}

export function updateCommunityUser(id: number | string, userId: number | string, permission: 'ADMIN' | 'USER') {
    return put(`community/${id}/user/${userId}`, { permission });
}
