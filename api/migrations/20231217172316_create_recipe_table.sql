create table recipe (
    id int not null auto_increment,
    title varchar(255) not null,
    description text,
    ingredients JSON not null,
    method JSON not null,
    user_id int not null,
    primary key (id),
    foreign key (user_id) references user(id) on delete cascade
)