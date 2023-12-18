create table recipe_post (
    recipe_id int not null,
    post_id int not null,
    primary key (recipe_id, post_id),
    foreign key (recipe_id) references recipe(id) on delete cascade,
    foreign key (post_id) references post(id) on delete cascade
)