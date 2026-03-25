CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS eggs (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  idx SERIAL,
  title TEXT NOT NULL,
  content_uri TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  email TEXT UNIQUE NOT NULL,
  name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS user_eggs (
  user_id UUID REFERENCES users(id),
  egg_id UUID REFERENCES eggs(id),
  PRIMARY KEY (user_id, egg_id)
);

-- Bed
INSERT INTO eggs (title, content_uri) VALUES ('Achievement Unlocked!', 'egg01.html');

-- Bowl
INSERT INTO eggs (title, content_uri) VALUES ('Breakfast of Champions', 'egg02.html');

-- Fortitude
INSERT INTO eggs (title, content_uri) VALUES ('Caffeine Buzzkiller', 'egg03.html');

-- Bike
INSERT INTO eggs (title, content_uri) VALUES ('The Wheels on the Bike Go Round and Round', 'egg04.html');

-- Parakeets
INSERT INTO eggs (title, content_uri) VALUES ('Don''t Bite the Hand that Feeds You', 'egg05.html');

-- Clock tower
INSERT INTO eggs (title, content_uri) VALUES ('Sir Benjamin the Big', 'egg06.html');

-- Pub
INSERT INTO eggs (title, content_uri) VALUES ('Three Cheers for Friendship!', 'egg07.html');

-- Under bed
INSERT INTO eggs (title, content_uri) VALUES ('CONGRATULATIONS!', 'egg08.html');
