import { get, post, put, remove } from '$lib/apiFetch';
import type { Recipe } from '$lib/app/recipe';

export type Cookbook = {
    id: number,
    title: string,
    description?: string,
    user_id: number
}

export function getCookbook(id: number | string) {
    return get<Cookbook>(`cookbook/${id}`)
}

export function createCookbook(title: string, description?: string) {
    return post(`cookbook`, { title, description })
}

export function getUserCookbooks(userId: number | string) {
    return get<Cookbook[]>(`cookbook/user/${userId}`);
}

export function updateCookbook(id: number | string, title: string, description?: string) {
    return put(`cookbook/${id}`, { title, description });
}

export function deleteCookbook(id: number | string) {
    return remove(`cookbook/${id}`);
}

export function getCookbookRecipes(id: number | string) {
    return get<Recipe[]>(`cookbook/${id}/recipe`);
}

export function addCookbookRecipe(id: number | string, recipeId: number | string) {
    return post(`cookbook/${id}/recipe/${recipeId}`);
}

export function removeCookbookRecipe(id: number | string, recipeId: number | string) {
    return remove(`cookbook/${id}/recipe/${recipeId}`);
}
