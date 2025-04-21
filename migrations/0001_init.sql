CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS eggs (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  idx SERIAL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  image_uri TEXT NOT NULL
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
INSERT INTO eggs (title, image_uri, description) VALUES (
  'Achievement Unlocked!',
  '/static/images/trophy.svg',
  '<p>You''ve discovered your first Easter egg! How many remain? I suppose we''ll just have to find out together.</p>
  <p>Each of these eggs will provide you with a hint to find the next one, so be sure to read them carefully.</p>
  <p>Get yourself ready for a day of adventure. This quest will have you searching high and low, near and far, inward and outward, squidward and scoutward.</p>
  <p>Erm... Ignore that last part.</p>
  <p>Get dressed and grab something to eat and drink. You''ll need the energy. It never did anybody any good to set out for adventure on an empty stomach!</p>
  <p>Might I recommend a bowl of cereal, or perhaps oatmeal?</p>'
);

-- Bowl
INSERT INTO eggs (title, image_uri, description) VALUES (
  'Breakfast of Champions',
  '/static/images/oatmeal.svg',
  '<p>There really is nothing like a good bowl of cereal in the morning. Some say it is not the most healthy, but I say let the haters hate.</p>
  <p>Cereal is good old reliable. A man''s breakfast. The original breakfast of champions. It''s scientifically proven to be the best way to start the day for growing boys and girls.</p>
  <p>A bowl of cereal first thing in the morning fills the muscles with strength and the brain with fortitude, both of which are necessary to become a champion.</p>
  <p>There is one other thing however that many consider to be essential in the morning, and that would be a warm cup of coffee. Second only to a bowl of cereal, coffee is the true energizer.</p>
  <p>When paired, the two work in harmony to kick start one''s day more effectively than any alternative.</p>
  <p>Why don''t you go ahead and grab a nice coffee? If not for yourself, then at least for your absolutely caffeine-addicted partner.</p>
  <p>Look to him now. Does he seem alright to you? No! He''s practically shivering, itching for his morning coffee! He needs it now, there''s no time to waste!</p>
  <p>It doesn''t need to be the most prestigious coffee joint in town, so long as it is close and quick to get to. Now get to it!</p>
  <p><small>PS. Maybe grab some things on the way out. You know, the essentials. A book, some snacks perhaps...</small></p>'
);

-- Fortitude
INSERT INTO eggs (title, image_uri, description) VALUES (
  'Caffeine Buzzkiller',
  '/static/images/coffee.svg',
  '<p>That was a really close call. Any longer and the caffeine headache would have started creeping in.</p>
  <p>Unfortunately now that we''ve had our coffee, we''ve got much too much energy. We''ll have to do something about that.</p>
  <p>Probably the most sensible thing would be to just go somewhere. Anywhere should be fine, we work out the destination after we get going.</p>
  <p>One thing''s for certain though and that is that we''ll need wheels. The neighbourhood is fine but who wants to waste Easter day traipsing around the same old haunts as usual.</p>
  <p>No, that simply won''t do. Today, we follow Gandalph''s instructions, the fools we are. Today, we fly!</p>
  <p><small>But not actually fly though... Seriously, that was not a hint or anything, that was just goofiness.</small></p>'
);

-- Bike
INSERT INTO eggs (title, image_uri, description) VALUES (
  'The Wheels on the Bike Go Round and Round',
  '/static/images/bike.svg',
  '<p>Hey, what do you know? Turns out that fetching a bike killed two birds with one stone. Congratulations! Although, we don''t condone killing birds around here... If anything, we prefer to love and feed them.</p>
  <p>I hope by now you''ve decided on a destination... No? You haven''t? Well luckily for you, I happened to use the walking time to think of one for you. Just in case you didn''t. As a backup plan.</p>
  <p>I thought perhaps a park picnic could be nice. It doesn''t have to be in the biggest or nicest park. Perhaps one nearly large enough for a game of hide and seek, but not quite.</p>
  <p>The name''s escaping me for the moment, but I know you know what I mean, and I know you''l find the perfect spot to feed.</p>'
);

-- Parakeets
INSERT INTO eggs (title, image_uri, description) VALUES (
  'Don''t Bite the Hand that Feeds You',
  '/static/images/parakeet.svg',
  '<p>Well that sure was a lot of fun, but oh boy, look at the time! It''s starting to get rather late and there are still eggs to collect!</p>
  <p>There''s so much to do and not enough time. At least we had a good time. I''m just trying to make sure we get home in time for the TV room (which has been reserved, by the way).</p>
  <p>Planning and designing this scavenger hunt has been a real challenge I''ll tell you. It''s been especially a challenge to strike the right balance of difficulty and bridge the gap between creative and cheesy.</p>
  <p>Not to mention, I almost didn''t deliver on time.</p>'
);

-- Clock tower
INSERT INTO eggs (title, image_uri, description) VALUES (
  'Sir Benjamin the Big',
  '/static/images/clocktower.svg',
  '<p>You know, one of my dear old friends'' name is Ben. He wasn''t really all that big, but he had a big heart at least.</p>
  <p>I sure do miss him.</p>
  <p>It''s easy to take our friendships for granted, you see. One minute, you''re growing up with a big circle and all the support you could ask for, close at hand.</p>
  <p>The next minute, everyone''s gone away getting married, starting families, and just living lives. Darn it, I''m starting to get teary eyed.</p>
  <p>Forget this scavenger hunt. I need a drink.</p>'
);

-- Pub
INSERT INTO eggs (title, image_uri, description) VALUES (
  'Three Cheers for Friendship!',
  '/static/images/cheers.svg',
  '<p>Well, it looks like we''re almost back where we started. It''s been quite the journey, physically <em>and</em> emotionally.</p>
  <p>It would seem some gratitude on my part is in order.</p>
  <p>Thank you for embarking on this wonderful challenge together with me. I hope you had many laughs and made many new memories from it.</p>
  <p>For now, perhaps it''s time to go back home and take a little nap. I can''t speak for you, but I''m so exhausted I could sink right through the bed and into the floor below.</p>
  <p>Just promise me that if that did really happen, you''d rescue me from under there. I''ve been seeing way too many reels lately of dumb kids folding each other up into retractable couch-beds, and I''m starting to fear our very own murphy bed a bit...</p>'
);

-- Under bed
INSERT INTO eggs (title, image_uri, description) VALUES (
  'CONGRATULATIONS!',
  '/static/images/easter.svg',
  '<p>You did it! You truly found the last egg!</p>
  <p>This warrants a prize, a much grander prize than I have in mind now. Perhaps the prize can be negotiated on. You decide what you want, and it will be so!</p>
  <p>I truly hope you enjoyed this quest. Until next year.</p>
  <h2>Happy Easter!</h2>'
);
