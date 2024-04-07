import { put, remove, post, get } from '$lib/apiFetch';

export type Comment = {
    id: number,
    content: string,
    user_id: number,
    user_display_name: string,
    reply_id: number | null,
    created: string,
    is_liked: number,
    likes: number
}

export function updateComment(id: number | string, content: string) {
    return put(`comment/${id}`, { content });
}

export function deleteComment(id: number | string) {
    return remove(`comment/${id}`);
}

export function likeComment(id: number | string) {
    return post(`comment/${id}/like`);
}

export function unlikeComment(id: number | string) {
    return remove(`comment/${id}/like`);
}

export function replyToComment(id: number | string, content: string) {
    return post(`comment`, { content, reply_id: id } );
}

export function getCommentReplies(id: number | string) {
    return get<Comment[]>(`comment/${id}/replies`);
}
