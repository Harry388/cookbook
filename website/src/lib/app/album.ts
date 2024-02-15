import { get, put, post, remove } from '$lib/apiFetch';
import type { Post } from '$lib/app/post';
import type { Recipe } from '$lib/app/recipe';

export type Album = {
    id: number,
    title: string,
    user_id: number
};

export type Entries = {
    posts: Post[],
    recipes: Recipe[]
};

export function getAlbum(id: number | string) {
    return get<Album>(`album/${id}`);
}

export function createAlbum(title: string) {
    return post(`album`, { title });
}

export function updateAlbum(id: number | string, title: string) {
    return put(`album/${id}`, { title });
}

export function deleteAlbum(id: number | string) {
    return remove(`album/${id}`);
}

export function getUserAblums(userId: number | string) {
    return get<Album[]>(`album/user/${userId}`);
}

export function getAlbumEntries(id: number | string) {
    return get<Entries>(`album/${id}/contents`);
}

export function addAlbumEntry(id: number | string, entryId: number | string, type: 'post' | 'recipe') {
    return post(`album/${id}/${type}/${entryId}`);
}

export function removeAlbumEntry(id: number | string, entryId: number | string, type: 'post' | 'recipe') {
    return remove(`album/${id}/${type}/${entryId}`);
}
