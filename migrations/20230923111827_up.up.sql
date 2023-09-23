CREATE TABLE IF NOT EXISTS dogs (
	id serial PRIMARY KEY,
	name VARCHAR NOT NULL,
	age INT NOT NULL
);

CREATE OR REPLACE FUNCTION notify_dogs_changes()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify(
    'dogs_changed',
    json_build_object(
      'operation', TG_OP,
      'record', row_to_json(NEW)
    )::text
  );


  RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER dogs_changed
AFTER INSERT OR UPDATE
ON dogs
FOR EACH ROW
EXECUTE PROCEDURE notify_dogs_changes()
