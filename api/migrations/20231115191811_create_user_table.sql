create table user (
    id int primary key not null auto_increment,
    username varchar(255) not null unique,
    display_name varchar(255) not null,
    email varchar(255) not null unique,
    password varchar(255) not null,
    bio varchar(255),
    pfp varchar(255),
    public boolean not null
)