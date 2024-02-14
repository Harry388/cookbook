create table album_entry (
    album_id int not null,
    post_id int,
    recipe_id int,
    primary key(album_id, post_id, recipe_id),
    foreign key(album_id) references album(id),
    foreign key(post_id) references post(id),
    foreign key(recipe_id) references recipe(id)
)
