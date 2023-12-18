import { get, post, remove, put } from '$lib/apiFetch';
import type { FetchFn } from '$lib/apiFetch';

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

export async function getRecipe(id: number | string, fetch?: FetchFn): Promise<Recipe> {
    const response = await get(`recipe/${id}`).run(fetch);
    const recipe: Recipe = await response.json();
    return recipe;
}

export async function getUserRecipes(userId: number | string, fetch?: FetchFn): Promise<Recipe[]> {
    const respose = await get(`recipe/user/${userId}`).run(fetch);
    const recipes: Recipe[] = await respose.json();
    return recipes;
}

export async function createRecipe(title: string, description: string | null, ingredients: Ingredients, method: Method, fetch?: FetchFn): Promise<Response> {
    return await post(`recipe`, { title, description, ingredients, method }).run(fetch);
}

export async function updateRecipe(id: number | string, title: string | null, description: string | null, ingredients: Ingredients | null, method: Method | null, fetch?: FetchFn): Promise<Response> {
    return await put(`recipe/${id}`, { title, description, ingredients, method }).run(fetch);
}

export async function deleteRecipe(id: number | string): Promise<Response> {
    return await remove(`recipe/${id}`).run(fetch);
}