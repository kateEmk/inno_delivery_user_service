ALTER TABLE users ADD COLUMN id serial UNIQUE;
ALTER TABLE courier ADD COLUMN id serial;
ALTER TABLE courier ADD COLUMN user_id integer null;

UPDATE courier c
SET id = u.id
FROM users u
WHERE c.uuid = u.uuid;

ALTER TABLE courier ALTER COLUMN id SET NOT NULL;

ALTER TABLE courier DROP CONSTRAINT courier_uuid_fkey;
ALTER TABLE courier ADD FOREIGN KEY(id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE users DROP COLUMN uuid;
ALTER TABLE courier DROP COLUMN uuid;

ALTER TABLE users ADD CONSTRAINT pk_id_users PRIMARY KEY(id);
ALTER TABLE courier ADD CONSTRAINT pk_id_courier PRIMARY KEY(id);