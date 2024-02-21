import { get } from '$lib/apiFetch';
import type { Post } from '$lib/app/post';
import type { Recipe } from '$lib/app/recipe';
import type { Follow } from '$lib/app/follow';
import type { Community } from '$lib/app/community';
import type { Tag } from '$lib/app/tag';

export function searchPosts(search: string) {
    return get<Post[]>(`search/post/${search}`);
}

export function searchRecipes(search: string) {
    return get<Recipe[]>(`search/recipe/${search}`);
}

export function searchCommunities(search: string) {
    return get<Community[]>(`search/community/${search}`);
}

export function searchTags(search: string) {
    return get<Tag[]>(`search/tag/${search}`);
}

export function searchUsers(search: string) {
    return get<Follow[]>(`search/user/${search}`);
}
