create table comment (
    id int not null auto_increment,
    content text not null,
    user_id int not null,
    reply_id int,
    primary key(id),
    foreign key(user_id) references user(id) on delete cascade,
    foreign key(reply_id) references comment(id) on delete cascade
)
