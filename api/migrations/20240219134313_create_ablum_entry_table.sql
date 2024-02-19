create table album_post (
    album_id int not null,
    post_id int not null,
    primary key(album_id, post_id),
    foreign key(album_id) references album(id) on delete cascade,
    foreign key(post_id) references post(id) on delete cascade
);

create table album_recipe (
    album_id int not null,
    recipe_id int not null,
    primary key(album_id, recipe_id),
    foreign key(album_id) references album(id) on delete cascade,
    foreign key(recipe_id) references recipe(id) on delete cascade
);
