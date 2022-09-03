CREATE TABLE rel_todos_tags (
  id CHAR(37) PRIMARY KEY,
  todo_id CHAR(37) NOT NULL,
  tag_id CHAR(37) NOT NULL,

  FOREIGN KEY(todo_id)
    REFERENCES todos(id),

  FOREIGN KEY(tag_id)
    REFERENCES tags(id)
); -- Your SQL goes here
