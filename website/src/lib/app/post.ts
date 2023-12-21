import { get, post, remove, put } from '$lib/apiFetch';
import type { FetchFn } from '$lib/apiFetch';
import type { Recipe } from '$lib/app/recipe';

export type Post = {
    id: number,
    title: string,
    content: string | null,
    user_id: number,
    media: number[],
    created: string
}

export async function getPost(id: number | string, fetch?: FetchFn): Promise<Post> {
    const response = await get(`post/${id}`).run(fetch);
    const post: Post = await response.json();
    return post;
}

export async function getUserPosts(userId: number | string, fetch?: FetchFn): Promise<Post[]> {
    const response = await get(`post/user/${userId}`).run(fetch);
    const posts: Post[] = await response.json();
    return posts;
}

export async function getPostRecipes(id: number | string, fetch?: FetchFn): Promise<Recipe[]> {
    const response = await get(`post/${id}/recipe`).run(fetch);
    const recipes: Recipe[] = await response.json();
    return recipes;
}

export async function createPost(title: string, content: string, files: File[], fetch?: FetchFn): Promise<Response> {
    const formData = new FormData();
    const postStr = JSON.stringify({ title, content });
    formData.append('post', postStr);
    if (files) {
        for (const file of files) {
            formData.append('media', file);
        }
    }
    return await post('post', formData, {
        headers: {
            'Content-Type':  'remove'
        }
    }).run(fetch);
}

export async function updatePost(id: number | string, title: string | null, content: string | null, fetch?: FetchFn): Promise<Response> {
    return await put(`post/${id}`, { title, content }).run(fetch);
}

export async function deletePost(id: number | string, fetch?: FetchFn): Promise<Response> {
    return await remove(`post/${id}`).run(fetch);
}