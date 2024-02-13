import { put, remove } from '$lib/apiFetch';

export type Comment = {
    id: number,
    content: string,
    user_id: number,
    user_display_name: string,
    reply_id: number | null,
    created: string
}

export function updateComment(id: number | string, content: string) {
    return put(`comment/${id}`, { content });
}

export function deleteComment(id: number | string) {
    return remove(`comment/${id}`);
}
