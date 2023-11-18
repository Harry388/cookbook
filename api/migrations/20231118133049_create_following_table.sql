create table following (
    user_id int not null,
    following_id int not null,
    primary key (user_id, following_id),
    foreign key (user_id) references user(id) on delete cascade,
    foreign key (following_id) references user(id) on delete cascade
)