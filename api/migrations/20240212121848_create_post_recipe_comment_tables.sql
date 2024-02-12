create table post_comment (
    post_id int not null,
    comment_id int not null,
    primary key(post_id, comment_id),
    foreign key(post_id) references post(id) on delete cascade,
    foreign key(comment_id) references comment(id) on delete cascade
);

create table recipe_comment (
    recipe_id int not null,
    comment_id int not null,
    primary key(recipe_id, comment_id),
    foreign key(recipe_id) references recipe(id) on delete cascade,
    foreign key(comment_id) references comment(id) on delete cascade
)
