CREATE TABLE chat_room (
  id SERIAL PRIMARY KEY,
  name VARCHAR
);

CREATE TABLE chat_history (
  msg_id SERIAL PRIMARY KEY,
  poster VARCHAR references users(id) NOT NULL,
  room INT references chat_room(id) NOT NULL,
  posted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  message TEXT NOT NULL
);

CREATE TABLE chat_occupants (
  room_id INT references chat_room(id) NOT NULL,
  user_id VARCHAR references users(id) NOT NULL,
  can_send BOOLEAN NOT NULL,
  can_add_user BOOLEAN NOT NULL,
  can_change_name BOOLEAN NOT NULL,
  last_received INT DEFAULT NULL,
  last_read INT DEFAULT NULL,
  PRIMARY KEY (room_id, user_id)
);

CREATE INDEX chat_occupants_idx on chat_occupants (room_id, user_id);
CREATE INDEX chat_history_idx on chat_history (room, posted_at);
