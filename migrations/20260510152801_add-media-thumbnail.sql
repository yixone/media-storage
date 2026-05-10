ALTER TABLE media
ADD COLUMN thumbnail_key VARCHAR(64) NOT NULL DEFAULT '';

UPDATE media
SET thumbnail_key = id
WHERE thumbnail_key = '';
