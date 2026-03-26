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
INSERT INTO eggs (id, title, content_uri) VALUES ('85160ada-30fd-4fc4-979c-008a10a18ff5', 'Achievement Unlocked!', 'egg01.html');

-- Bowl
INSERT INTO eggs (id, title, content_uri) VALUES ('72ed3d00-56d6-46eb-ac32-40c0c24ed6b7', 'Breakfast of Champions', 'egg02.html');

-- Fortitude
INSERT INTO eggs (id, title, content_uri) VALUES ('8011d3b1-b357-4f25-9272-795cca1086a4', 'Caffeine Buzzkiller', 'egg03.html');

-- Bike
INSERT INTO eggs (id, title, content_uri) VALUES ('d3aacfed-cb66-433b-aa8d-1d7c938f6cd0', 'The Wheels on the Bike Go Round and Round', 'egg04.html');

-- Parakeets
INSERT INTO eggs (id, title, content_uri) VALUES ('538d041c-8a3a-461b-8a1e-939ff512e2a8', 'Don''t Bite the Hand that Feeds You', 'egg05.html');

-- Clock tower
INSERT INTO eggs (id, title, content_uri) VALUES ('218c50bd-b436-4153-99a5-4f84363e1dbd', 'Sir Benjamin the Big', 'egg06.html');

-- Pub
INSERT INTO eggs (id, title, content_uri) VALUES ('e43c8174-d252-4ac7-8f39-c420d1584390', 'Three Cheers for Friendship!', 'egg07.html');

-- Under bed
INSERT INTO eggs (id, title, content_uri) VALUES ('fb0ae7f9-6a51-4478-82fa-0860e0f1d0aa', 'CONGRATULATIONS!', 'egg08.html');
