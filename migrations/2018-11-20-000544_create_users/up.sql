-- Your SQL goes here
CREATE TABLE users(
  id SERIAL PRIMARY KEY,
  username VARCHAR (60) NOT NULL,
  email VARCHAR (60) NOT NULL
);
