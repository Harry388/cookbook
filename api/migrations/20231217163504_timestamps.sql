alter table user add created timestamp not null default current_timestamp;
alter table post add created timestamp not null default current_timestamp;