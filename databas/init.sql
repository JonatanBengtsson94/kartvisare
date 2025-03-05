CREATE TABLE wms_services (
  layer_id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  url VARCHAR(255) NOT NULL,
  version VARCHAR(5),
  is_active BOOLEAN DEFAULT TRUE,
  auth_type VARCHAR(50),
  auth_username TEXT,
  auth_password TEXT,
);

CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  username VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE user_groups (
  group_id SERIAL PRIMARY KEY,
  group_name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE user_group_membership (
  user_id INT REFERENCES users(user_id) ON DELETE CASCADE,
  group_id INT REFERENCES user_groups(group_id) ON DELETE CASCADE,
  PRIMARY KEY (user_id, group_id)
);

CREATE TABLE sessions (
  session_token UUID PRIMARY KEY,
  user_id INT REFERENCES users(user_id) ON DELETE CASCADE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  expires_at TIMESTAMP
);
