CREATE TABLE wms_groups (
  group_id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  parent_id INTEGER REFERENCES wms_groups(group_id) ON DELETE CASCADE
);

CREATE TABLE wms (
  wms_id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  layers TEXT[] NOT NULL CHECK (array_length(layers, 1) > 0),
  url VARCHAR(255) NOT NULL,
  version VARCHAR(5),
  is_active BOOLEAN DEFAULT TRUE,
  auth_type VARCHAR(50),
  auth_username TEXT,
  auth_password TEXT,
  group_id INTEGER REFERENCES wms_groups(group_id) ON DELETE SET NULL
);

CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  user_name VARCHAR(255) NOT NULL UNIQUE,
  idp_id VARCHAR(255)
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

CREATE TABLE wms_user_groups_membership (
  wms_id INT REFERENCES wms(wms_id) ON DELETE CASCADE,
  group_id INT REFERENCES user_groups(group_id) ON DELETE CASCADE,
  PRIMARY KEY (wms_id, group_id)
);

INSERT INTO user_groups (group_name) VALUES ('Admin');
INSERT INTO users (user_name, idp_id) VALUES ('kartvisare', 'kartvisare');
