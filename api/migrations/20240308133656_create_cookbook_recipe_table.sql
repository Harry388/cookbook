create table cookbook_recipe (
    recipe_id int not null,
    cookbook_id int not null,
    position int not null,
    primary key (recipe_id, cookbook_id),
    foreign key (recipe_id) references recipe(id),
    foreign key (cookbook_id) references cookbook(id)
)
