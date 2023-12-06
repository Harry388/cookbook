create table post (
    id int not null auto_increment,
    title varchar(255) not null,
    content text,
    user_id int not null,
    primary key (id),
    foreign key (user_id) references user(id) on delete cascade
);

create table post_media (
    id int not null auto_increment,
    uri varchar(255) not null,
    post_id int not null,
    primary key (id),
    foreign key (post_id) references post(id) on delete cascade
)