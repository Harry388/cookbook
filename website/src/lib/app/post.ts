import { get, post, remove, put } from '$lib/apiFetch';
import { getCommunity } from '$lib/app/community';
import { getUser } from '$lib/app/user';
import type { FetchFn } from '$lib/apiFetch';
import type { Recipe } from '$lib/app/recipe';
import type { Community } from '$lib/app/community';
import type { User } from '$lib/app/user';

export type Post = {
    id: number,
    title: string,
    content: string | null,
    user_id: number,
    community_id: number,
    media: number[],
    created: string
}

export type PostFull = Post & {
    community: Community | null,
    user: User
}

export async function getPost(id: number | string, fetch?: FetchFn): Promise<PostFull> {
    const response = await get(`post/${id}`).run(fetch);
    const post: Post = await response.json();
    const [community, user] = await Promise.all([post.community_id ? getCommunity(post.community_id, fetch) : null, getUser(post.user_id, fetch)]);
    return {
        ...post,
        community,
        user
    };
}

export async function getUserPosts(userId: number | string, fetch?: FetchFn): Promise<PostFull[]> {
    const response = await get(`post/user/${userId}`).run(fetch);
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

export async function createPost(title: string, content: string, communityId: number | null, files: File[], fetch?: FetchFn): Promise<Response> {
    const formData = new FormData();
    const postStr = JSON.stringify({ title, content, community_id: communityId });
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

export async function getPostRecipes(id: number | string, fetch?: FetchFn): Promise<Recipe[]> {
    const response = await get(`post/${id}/recipe`).run(fetch);
    const recipes: Recipe[] = await response.json();
    return recipes;
}

export async function addPostRecipe(id: number | string, recipeId: number | string, fetch?: FetchFn): Promise<Response> {
    return await post(`post/${id}/addrecipe/${recipeId}`).run(fetch);
}

export async function deletePostRecipe(id: number | string, recipeId: number | string, fetch?: FetchFn): Promise<Response> {
    return await remove(`post/${id}/removerecipe/${recipeId}`).run(fetch);
}
