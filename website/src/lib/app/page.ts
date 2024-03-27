import type { Section } from '$lib/app/cookbook';
import type { Recipe } from '$lib/app/recipe';

export type Page = 
(Section
& {
    type: 'Section'
}) |
(Recipe
& {
    type: 'Recipe'
});
