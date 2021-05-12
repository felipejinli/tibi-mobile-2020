CREATE TABLE images (
  id VARCHAR PRIMARY KEY,
  private BOOLEAN NOT NULL,
  created_by VARCHAR NOT NULL references users(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  original_size_bytes INT NOT NULL CHECK (original_size_bytes > 0),
  optimised_size_bytes INT NOT NULL CHECK (optimised_size_bytes > 0)
);

create TABLE image_permissions (
  image_id VARCHAR NOT NULL references images(id),
  user_id VARCHAR NOT NULL references users(id),
  can_write BOOLEAN NOT NULL,
  can_read BOOLEAN NOT NULL,
  can_remove BOOLEAN NOT NULL,
  PRIMARY KEY (image_id, user_id)
);
