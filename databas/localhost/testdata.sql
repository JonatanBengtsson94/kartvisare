INSERT INTO wms_groups (name) VALUES ('World');
INSERT INTO wms_groups (name, parent_id) VALUES ('Usa', 1);
INSERT INTO wms_groups (name, parent_id) VALUES ('Europe', 1);

INSERT INTO wms (name, layers, url, is_active, group_id) 
VALUES ('States', ARRAY['topp:states'], 'http://localhost:8001/geoserver/wms', true, 2);

