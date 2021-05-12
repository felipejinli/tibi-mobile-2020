CREATE TYPE Lineupitem As (
  label VARCHAR,
  description VARCHAR
);

CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  pre_title VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  location VARCHAR NOT NULL, -- Physical location if is virtual: false, otherwise the method e.g. Zoom
  is_virtual BOOLEAN NOT NULL,
  virtual_link VARCHAR,
  price_pence INT NOT NULL,
  images text[] NOT NULL,
  lineup Lineupitem[] NOT NULL,
  visible BOOLEAN NOT NULL DEFAULT true,
  event_start TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- CREATE TABLE event_lineup (
--   id SERIAL PRIMARY KEY,
--   event INT references events(id),
--   label VARCHAR,
--   description VARCHAR
-- );
