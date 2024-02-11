import { get, post, remove, put } from '$lib/apiFetch';
import type { Recipe } from '$lib/app/recipe';

export type Post = {
    id: number,
    title: string,
    content: string | null,
    user_id: number,
    user_display_name: string,
    community_id: number | null,
    community_title: string | null,
    media: number[],
    created: string
}

export function getPost(id: number | string) {
     return get<Post>(`post/${id}`);
}

export function getUserPosts(userId: number | string) {
    return get<Post[]>(`post/user/${userId}`);
}

export function createPost(title: string, content: string, communityId: number | null, files: File[]) {
    const formData = new FormData();
    const postStr = JSON.stringify({ title, content, community_id: communityId });
    formData.append('post', postStr);
    if (files) {
        for (const file of files) {
            formData.append('media', file);
        }
    }
    return post('post', formData, {
        headers: {
            'Content-Type':  'remove'
        }
    });
}

export function updatePost(id: number | string, title: string | null, content: string | null) {
    return put(`post/${id}`, { title, content });
}

export function deletePost(id: number | string) {
    return remove(`post/${id}`);
}

export function getPostRecipes(id: number | string) {
    return get<Recipe[]>(`post/${id}/recipe`);
}

export function addPostRecipe(id: number | string, recipeId: number | string) {
    return post(`post/${id}/addrecipe/${recipeId}`);
}

export function deletePostRecipe(id: number | string, recipeId: number | string) {
    return remove(`post/${id}/removerecipe/${recipeId}`);
}
