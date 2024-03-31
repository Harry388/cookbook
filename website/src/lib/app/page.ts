import type { Section } from '$lib/app/cookbook';
import type { Recipe } from '$lib/app/recipe';

export type Page = PageSection | PageRecipe | PageImage;

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

type PageImage = {
    image: string,
    type: 'Image'
};

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
        else if ((section != null) && (page.type == 'Recipe')) {
            section.recipes.push(page);
        }
        n++;
    }
    if (section != null) {
        book.push(section);
    }
    return book;
}
