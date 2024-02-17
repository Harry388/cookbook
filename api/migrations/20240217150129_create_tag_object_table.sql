create table tag_entry (
    id int not null auto_increment,
    tag_id int not null,
    post_id int,
    recipe_id int,
    primary key(id),
    foreign key(tag_id) references tag(id) on delete cascade,
    foreign key(post_id) references post(id) on delete cascade,
    foreign key(recipe_id) references recipe(id) on delete cascade
)
