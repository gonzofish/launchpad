-- Your SQL goes here
CREATE TABLE projects (
  id SERIAL PRIMARY KEY,
  end_date DATE,
  start_date DATE NOT NULL,
  title VARCHAR NOT NULL
);
