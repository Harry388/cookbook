create table community_user (
    user_id int not null,
    community_id int not null,
    permission enum('ADMIN', 'USER') not null,
    primary key (user_id, community_id),
    foreign key (user_id) references user(id) on delete cascade,
    foreign key (community_id) references community(id) on delete cascade
)

