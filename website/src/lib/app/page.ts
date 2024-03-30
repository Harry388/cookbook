import type { Section } from '$lib/app/cookbook';
import type { Recipe } from '$lib/app/recipe';

export type Page = PageSection | PageRecipe;

type PageSection = 
(Section
& {
    type: 'Section'
});

type PageRecipe = 
(Recipe
& {
    type: 'Recipe'
});

export type BookSection = {
    section: PageSection,
    recipes: PageRecipe[]
}

export type Book = BookSection[];

export function formatPageArray(pages: Page[]): Book {
    const book: Book = [];
    let section = null;
    for (const page of pages) {
        if (page.type == 'Section') {
            if (section != null) {
                book.push(section);
            }
            section = { section: page, recipes: Array<PageRecipe>() };
        }
        else if (section != null) {
            section.recipes.push(page);
        }
    }
    if (section != null) {
        book.push(section);
    }
    return book;
}
