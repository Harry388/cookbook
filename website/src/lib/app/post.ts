import { get, post } from '$lib/apiFetch';
import type { FetchFn } from '$lib/apiFetch';

export type Post = {
    id: number,
    title: string,
    content: string | null,
    user_id: number,
    media: number[]
}

export async function getUserPosts(userId: number | string, fetch?: FetchFn): Promise<Post[]> {
    const response = await get(`post/user/${userId}`).run(fetch);
    const posts: Post[] = await response.json();
    return posts;
}

export async function createPost(title: string, content: string, files: FileList, fetch?: FetchFn) {
    const formData = new FormData();
    const postStr = JSON.stringify({ title, content });
    formData.append('post', postStr);
    if (files) {
        for (const file of files) {
            formData.append('media', file);
        }
    }
    await post('post', formData, {
        headers: {
            'Content-Type':  'remove'
        }
    }).run(fetch);
}