import { get, post, remove } from '$lib/apiFetch';
import type { Recipe } from '$lib/app/recipe';
import type { Post } from '$lib/app/post';

export type Tag = {
    id: number,
    tag: string
};

export type Entries = {
    posts: Post[],
    recipes: Recipe[]
}

export function getTagEntries(id: number | string) {
    return get<Entries>(`tag/${id}/entries`);
}

export function getEntryTags(entry_id: number | string, type: 'post' | 'recipe') {
    return get<Tag[]>(`tag/entry/${type}/${entry_id}`);
}

export function addEntryTags(entry_id: number | string, tags: string[], type: 'post' | 'recipe') {
    return post(`tag/entry/${type}/${entry_id}`, tags);
}

export function removeEntryTags(entry_id: number | string, tags: number[], type: 'post' | 'recipe') {
    return remove(`tag/entry/${type}/${entry_id}`, tags);
}
