create table post_tag (
    post_id int not null,
    tag_id int not null,
    primary key (post_id, tag_id),
    foreign key (post_id) references post(id) on delete cascade,
    foreign key (tag_id) references tag(id) on delete cascade
)
