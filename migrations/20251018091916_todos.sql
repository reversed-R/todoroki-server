CREATE TABLE todos (
  id UUID PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  is_public BOOLEAN NOT NULL,
  alternative_name TEXT DEFAULT NULL,
  started_at TIMESTAMPTZ DEFAULT NULL,
  scheduled_at TIMESTAMPTZ DEFAULT NULL,
  ended_at TIMESTAMPTZ DEFAULT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE labels (
  id UUID PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE todo_labels (
  todo_id UUID REFERENCES todos(id) ON DELETE CASCADE,
  label_id UUID REFERENCES labels(id) ON DELETE CASCADE,
  PRIMARY KEY (todo_id, label_id)
);

/*
// TRIGGERS
*/
CREATE FUNCTION refresh_updated_at_step1() RETURNS trigger AS
$$
BEGIN
  IF NEW.updated_at = OLD.updated_at THEN
    NEW.updated_at := NULL;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
    
CREATE FUNCTION refresh_updated_at_step2() RETURNS trigger AS
$$
BEGIN
  IF NEW.updated_at IS NULL THEN
    NEW.updated_at := OLD.updated_at;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION refresh_updated_at_step3() RETURNS trigger AS
$$
BEGIN
  IF NEW.updated_at IS NULL THEN
    NEW.updated_at := CURRENT_TIMESTAMP;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/*
// TRIGGERS (todos)
*/
CREATE TRIGGER refresh_todos_updated_at_step1
    BEFORE UPDATE ON todos FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_todos_updated_at_step2
    BEFORE UPDATE OF updated_at ON todos FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_todos_updated_at_step3
    BEFORE UPDATE ON todos FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step3();

/*
// TRIGGERS (labels)
*/
CREATE TRIGGER refresh_labels_updated_at_step1
    BEFORE UPDATE ON labels FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_labels_updated_at_step2
    BEFORE UPDATE OF updated_at ON labels FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_labels_updated_at_step3
    BEFORE UPDATE ON labels FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step3();
