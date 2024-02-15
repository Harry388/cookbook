create table album_entry (
    id int not null auto_increment,
    album_id int not null,
    post_id int,
    recipe_id int,
    primary key(id),
    foreign key(album_id) references album(id) on delete cascade,
    foreign key(post_id) references post(id) on delete cascade,
    foreign key(recipe_id) references recipe(id) on delete cascade
)
