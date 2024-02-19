create table post_like (
    post_id int not null,
    user_id int not null,
    primary key(post_id, user_id),
    foreign key(post_id) references post(id) on delete cascade,
    foreign key(user_id) references user(id) on delete cascade
);

create table recipe_like (
    recipe_id int not null,
    user_id int not null,
    primary key(recipe_id, user_id),
    foreign key(recipe_id) references recipe(id) on delete cascade,
    foreign key(user_id) references user(id) on delete cascade
);

create table comment_like (
    comment_id int not null,
    user_id int not null,
    primary key(comment_id, user_id),
    foreign key(comment_id) references comment(id) on delete cascade,
    foreign key(user_id) references user(id) on delete cascade
);
