create table cookbook_recipe (
    recipe_id int not null,
    section_id int not null,
    position int not null,
    primary key (recipe_id, section_id),
    foreign key (recipe_id) references recipe(id) on delete cascade,
    foreign key (section_id) references cookbook_section(id) on delete cascade
)
