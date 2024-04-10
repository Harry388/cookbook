alter table post_like add created timestamp not null default current_timestamp;
alter table recipe_like add created timestamp not null default current_timestamp;
alter table comment_like add created timestamp not null default current_timestamp;
