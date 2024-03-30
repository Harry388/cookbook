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

type Book = BookSection[];

export function formatPageArray(pages: Page[]): Book {
    const book: Book = [];
    let section = null;
    let n = 0;
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
        n++;
    }
    if (section != null) {
        book.push(section);
    }
    return book;
}
