alter table post
    add community_id int,
    add constraint community foreign key(community_id) references community(id) on delete set null;
