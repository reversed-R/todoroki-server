CREATE TYPE todo_schedule_interval AS ENUM ('once', 'daily', 'weekly', 'monthly');

CREATE TABLE todo_schedules (
  todo_id UUID REFERENCES todos(id) ON DELETE CASCADE,
  interval todo_schedule_interval NOT NULL,
  starts_at TIMESTAMPTZ NOT NULL,
  ends_at TIMESTAMPTZ NOT NULL
);
