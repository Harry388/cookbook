import { get, post, put, remove } from '$lib/apiFetch';
import type { Page } from '$lib/app/page';

export type Cookbook = {
    id: number,
    title: string,
    description?: string,
    user_id: number,
    user_display_name: string
}

export type Section = {
    id: number,
    position: number,
    description?: string,
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

export function updateCookbookSection(id: number | string, section_id: number | string, title: string, description: string) {
    return put(`cookbook/${id}/section/${section_id}`, { title, description } );
}

export function removeCookbookSection(id: number | string, section_id: number | string) {
    return remove(`cookbook/${id}/section/${section_id}`);
}

export function getCookbookPages(id: number | string) {
    return get<Page[]>(`cookbook/${id}/pages`);
}

export function setCookbookRecipePic(id: number | string, sectionId: number | string, recipeId: number | string, pic: File) {
    const formData = new FormData();
    formData.append('image', pic);
    return put(`cookbook/${id}/section/${sectionId}/recipe/${recipeId}/image`, formData, {
        headers: {
            'Content-Type': 'remove'
        }
    });
}

export function removeCookbookRecipePic(id: number | string, sectionId: number | string, recipeId: number | string) {
    return remove(`cookbook/${id}/section/${sectionId}/recipe/${recipeId}/image`);
}
