create table tag_post (
    tag_id int not null,
    post_id int not null,
    primary key(tag_id, post_id),
    foreign key(tag_id) references tag(id) on delete cascade,
    foreign key(post_id) references post(id) on delete cascade
);

create table tag_recipe (
    tag_id int not null,
    recipe_id int not null,
    primary key(tag_id, recipe_id),
    foreign key(tag_id) references tag(id) on delete cascade,
    foreign key(recipe_id) references recipe(id) on delete cascade
);
