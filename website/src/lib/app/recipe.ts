import { get, post, remove, put } from '$lib/apiFetch';
import type { Post } from '$lib/app/post';

export type Ingredients = string[];

export type Method = string[]; 

export type Recipe = {
    id: number,
    title: string,
    description: string | null,
    user_id: number,
    user_display_name: string,
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
    return remove(`recipe/${id}`);
}

export function getRecipePosts(id: number | string) {
    return get<Post[]>(`recipe/${id}/post`);
}
