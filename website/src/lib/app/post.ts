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

export function getPost(id: number | string) {
    return {
        async json(fetch?: FetchFn) {
            const post = await get<Post>(`post/${id}`).json(fetch);
            const [community, user] = await Promise.all([post.community_id ? getCommunity(post.community_id).json(fetch) : null, getUser(post.user_id).json(fetch)]);
            return {
                ...post,
                community,
                user
            };
        }
    }
}

export function getUserPosts(userId: number | string) {
    return {
        async json(fetch?: FetchFn) {
            const posts = await get<Post[]>(`post/user/${userId}`).json(fetch);
            const postsFull: PostFull[] = [];
            for (const post of posts) {
                const [community, user] = await Promise.all([post.community_id ? getCommunity(post.community_id).json(fetch) : null, getUser(post.user_id).json(fetch)]);
                postsFull.push({
                    ...post,
                    community,
                    user
                });
            }
            return postsFull;
        }
    }
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
