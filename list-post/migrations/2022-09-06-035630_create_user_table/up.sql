-- Table: main.users
CREATE TABLE IF NOT EXISTS main.users
(
  id         SERIAL PRIMARY KEY,
  title      VARCHAR                  NOT NULL,
  body       TEXT                     NOT NULL,
  published  BOOLEAN                  NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
)