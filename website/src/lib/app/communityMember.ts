import { get, post, remove, put } from '$lib/apiFetch';
import type { FetchFn } from '$lib/apiFetch';

export type CommunityMember = {
    id: number,
    username: string,
    display_name: string,
    permission: 'ADMIN' | 'USER'
}

export async function getCommunityMembers(id: number | string, fetch?: FetchFn): Promise<CommunityMember[]> {
    const response = await get(`community/${id}/members`).run(fetch);
    const members = await response.json();
    return members;
}
