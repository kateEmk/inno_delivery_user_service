ALTER TABLE users ADD COLUMN uuid uuid DEFAULT gen_random_uuid() UNIQUE;
ALTER TABLE courier ADD COLUMN uuid uuid NULL;

UPDATE courier c
SET uuid = u.uuid
FROM users u
WHERE c.id = u.id;

ALTER TABLE courier ALTER COLUMN uuid SET NOT NULL;

ALTER TABLE courier DROP CONSTRAINT courier_id_fkey;
ALTER TABLE courier ADD FOREIGN KEY(uuid) REFERENCES users(uuid) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE users DROP COLUMN id;
ALTER TABLE courier DROP COLUMN user_id;
ALTER TABLE courier DROP COLUMN id;

ALTER TABLE users ADD CONSTRAINT pk_uuid_users PRIMARY KEY(uuid);
ALTER TABLE courier ADD CONSTRAINT pk_uuid_courier PRIMARY KEY(uuid);