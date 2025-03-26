INSERT INTO wms_groups (name) VALUES ('World');
INSERT INTO wms_groups (name, parent_id) VALUES ('Usa', 1);
INSERT INTO wms_groups (name, parent_id) VALUES ('Europe', 1);

INSERT INTO users (user_name, idp_id) VALUES ('John', 'abc');
INSERT INTO users (user_name, idp_id) VALUES ('Jane', 'def');

INSERT INTO user_groups (group_name) VALUES ('Standard');

INSERT INTO user_group_membership (user_id, group_id) VALUES (1, 1);
INSERT INTO user_group_membership (user_id, group_id) VALUES (2, 2);

INSERT INTO wms (name, layers, url, is_active, group_id) 
VALUES ('States', ARRAY['topp:states'], 'http://localhost:8001/geoserver/wms', true, 2);

INSERT INTO wms (name, layers, url, is_active, group_id)
VALUES ('Lakes', ARRAY['topp:tasmania_water_bodies'], 'http://localhost:8001/geoserver/wms', true, 3);

INSERT INTO wms_user_groups_membership (wms_id, group_id) VALUES (1, 1);
INSERT INTO wms_user_groups_membership (wms_id, group_id) VALUES (2, 2);
