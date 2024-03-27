create table cookbook_section (
    id int not null auto_increment,
    title varchar(255) not null,
    position int not null,
    cookbook_id int not null,
    primary key (id),
    foreign key (cookbook_id) references cookbook(id) on delete cascade
)
