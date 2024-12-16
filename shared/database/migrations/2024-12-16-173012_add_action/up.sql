-- Your SQL goes here
INSERT INTO actions (api_service_id, name, description, endpoint, method, headers, params, json_path)
VALUES 
(1, 'New Mail Received (Gmail)', 'Checks if a new mail is received on Gmail', '/gmail/v1/users/me/messages', 'GET', '{"Authorization": "Bearer {token}"}', '{"maxResults": 1}', 'messages[0]'),
(2, 'New Mail Received (Outlook)', 'Checks if a new mail is received on Outlook', '/me/messages', 'GET', '{"Authorization": "Bearer {token}"}', '{"$top": 1, "$select": "receivedDateTime"}', 'value[0]');