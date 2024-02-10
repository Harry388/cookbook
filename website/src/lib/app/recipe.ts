import { get, post, remove, put } from '$lib/apiFetch';
import { getCommunity } from '$lib/app/community';
import { getUser } from '$lib/app/user';
import type { FetchFn } from '$lib/apiFetch';
import type { PostFull, Post } from '$lib/app/post';

export type Ingredients = string[];

export type Method = string[]; 

export type Recipe = {
    id: number,
    title: string,
    description: string | null,
    user_id: number,
    ingredients: Ingredients,
    method: Method,
    created: string
}

export function getRecipe(id: number | string) {
    return get<Recipe>(`recipe/${id}`);
}

export function getUserRecipes(userId: number | string) {
    return get<Recipe[]>(`recipe/user/${userId}`);
}

export function createRecipe(title: string, description: string | null, ingredients: Ingredients, method: Method) {
    return post(`recipe`, { title, description, ingredients, method });
}

export function updateRecipe(id: number | string, title: string | null, description: string | null, ingredients: Ingredients | null, method: Method | null) {
    return put(`recipe/${id}`, { title, description, ingredients, method });
}

export function deleteRecipe(id: number | string) {
    return  remove(`recipe/${id}`);
}

export async function getRecipePosts(id: number | string) {
    return {
        async json(fetch?: FetchFn) {
            const posts = await get<Post[]>(`recipe/${id}/post`).json(fetch);
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
