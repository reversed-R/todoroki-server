-- Add migration script here
CREATE TABLE doits (
  id UUID PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  is_public BOOLEAN NOT NULL,
  alternative_name TEXT DEFAULT NULL,
  deadlined_at TIMESTAMPTZ DEFAULT NULL,
  affects_to UUID DEFAULT NULL,
  created_by UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE doit_labels (
  doit_id UUID REFERENCES doits(id) ON DELETE CASCADE,
  label_id UUID REFERENCES labels(id) ON DELETE CASCADE,
  PRIMARY KEY (doit_id, label_id)
);


/*
// TRIGGERS (doits)
*/
CREATE TRIGGER refresh_doits_updated_at_step1
    BEFORE UPDATE ON doits FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_doits_updated_at_step2
    BEFORE UPDATE OF updated_at ON doits FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_doits_updated_at_step3
    BEFORE UPDATE ON doits FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step3();
