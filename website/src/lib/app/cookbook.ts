import { get, post, put, remove } from '$lib/apiFetch';
import type { Page } from '$lib/app/page';

export type Cookbook = {
    id: number,
    title: string,
    description?: string,
    user_id: number
}

export type Section = {
    id: number,
    position: number,
    title: string
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

export function addCookbookRecipe(id: number | string, sectionId: number | string, recipeId: number | string) {
    return post(`cookbook/${id}/section/${sectionId}/recipe/${recipeId}`);
}

export function removeCookbookRecipe(id: number | string, sectionId: number | string, recipeId: number | string) {
    return remove(`cookbook/${id}/section/${sectionId}/recipe/${recipeId}`);
}

export function addCookbookSection(id: number | string, title: string) {
    return post(`cookbook/${id}/section`, { title });
}

export function removeCookbookSection(id: number | string, section_id: number | string) {
    return remove(`cookbook/${id}/section/${section_id}`);
}

export function getCookbookPages(id: number | string) {
    return get<Page[]>(`cookbook/${id}/pages`);
}
