CREATE TYPE user_role AS ENUM ('owner', 'contributor');

CREATE TABLE users (
  id UUID PRIMARY KEY NOT NULL,
  role user_role NOT NULL DEFAULT 'contributor',
  name TEXT NOT NULL,
  email TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

/*
// TRIGGERS (todos)
*/
CREATE TRIGGER refresh_users_updated_at_step1
    BEFORE UPDATE ON users FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_users_updated_at_step2
    BEFORE UPDATE OF updated_at ON users FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_users_updated_at_step3
    BEFORE UPDATE ON users FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step3();
