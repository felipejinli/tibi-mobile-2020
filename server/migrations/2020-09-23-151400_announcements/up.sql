CREATE TABLE announcements (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  subtitle VARCHAR NOT NULL,
  image VARCHAR NOT NULL references images(id),
  visible BOOLEAN NOT NULL DEFAULT true,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
