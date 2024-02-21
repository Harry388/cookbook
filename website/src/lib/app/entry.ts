import type { Post } from '$lib/app/post';
import type { Recipe } from '$lib/app/recipe';

export type Entry = 
(Post
& {
    type: 'Post'
}) |
(Recipe
& {
    type: 'Recipe'
});
