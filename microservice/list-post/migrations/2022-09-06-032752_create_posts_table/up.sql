-- Table: main.posts
CREATE TABLE IF NOT EXISTS main.posts
(
  id         SERIAL PRIMARY KEY,
  title      VARCHAR                  NOT NULL,
  body       TEXT                     NOT NULL,
  published  BOOLEAN                  NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
)