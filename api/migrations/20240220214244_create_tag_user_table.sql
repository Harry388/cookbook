create table tag_user (
    tag_id int not null,
    user_id int not null,
    primary key(tag_id, user_id),
    foreign key(tag_id) references tag(id) on delete cascade,
    foreign key(user_id) references user(id) on delete cascade
)
