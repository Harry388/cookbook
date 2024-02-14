create table album (
    id int not null auto_increment,
    title varchar(255) not null,
    user_id int not null,
    primary key(id),
    foreign key(user_id) references user(id)
)
