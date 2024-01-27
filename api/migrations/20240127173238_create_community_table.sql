create table community (
    id int primary key not null auto_increment,
    title varchar(255) not null,
    description text,
    pfp varchar(255)
)
