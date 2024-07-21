-- Active: 1721553650458@@127.0.0.1@5432@disconnect-app-storage@public

insert into roles (name, uuid) values ('admin','41cf295e-3c4f-4f43-abc2-63e2b25c2ea2');

insert into roles (name, uuid) values ('fellow_read','915f0fe3-d333-4d7f-b2c9-ec0c70a36c8b');

insert into roles (name, uuid) values ('fellow_creator','ca38d4bf-9365-4eee-a7e1-bef9067f113c');

insert into roles (name, uuid) values ('member_read','457a7e33-c8b0-4013-8710-f869dad1cb89');

insert into roles (name, uuid) values ('member_creator','98672f3b-9296-49ea-a15c-8f98e6163994');

insert into systemusers (user_id, secret) values ('admin', 'for-now-plain-pwd');

insert into system_user_roles (system_user_id, role_id) values (1, 2);
insert into system_user_roles (system_user_id, role_id) values (1, 3);
insert into system_user_roles (system_user_id, role_id) values (1, 4);
insert into system_user_roles (system_user_id, role_id) values (1, 5);

commit;