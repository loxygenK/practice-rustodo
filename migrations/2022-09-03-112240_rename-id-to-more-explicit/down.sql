-- This file should undo anything in `up.sql`

ALTER TABLE todos RENAME COLUMN todo_id TO id;
ALTER TABLE tags RENAME COLUMN tag_id TO id;
