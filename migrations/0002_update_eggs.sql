ALTER TABLE eggs ADD content_uri TEXT;

UPDATE eggs SET content_uri = 'egg01.html' WHERE idx = 1;
UPDATE eggs SET content_uri = 'egg02.html' WHERE idx = 2;
UPDATE eggs SET content_uri = 'egg03.html' WHERE idx = 3;
UPDATE eggs SET content_uri = 'egg04.html' WHERE idx = 4;
UPDATE eggs SET content_uri = 'egg05.html' WHERE idx = 5;
UPDATE eggs SET content_uri = 'egg06.html' WHERE idx = 6;
UPDATE eggs SET content_uri = 'egg07.html' WHERE idx = 7;
UPDATE eggs SET content_uri = 'egg08.html' WHERE idx = 8;

ALTER TABLE eggs ALTER COLUMN content_uri SET NOT NULL;

ALTER TABLE eggs DROP COLUMN description;
ALTER TABLE eggs DROP COLUMN image_uri;
