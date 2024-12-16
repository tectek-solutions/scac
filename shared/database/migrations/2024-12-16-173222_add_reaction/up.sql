-- Your SQL goes here
INSERT INTO reactions (api_service_id, name, description, endpoint, method, headers, params, json_path)
VALUES 
(1, 'Send Mail (Gmail)', 'Sends a mail as the user on Gmail', '/gmail/v1/users/me/messages/send', 'POST', '{"Authorization": "Bearer {token}"}', '{}', 'raw'),
(2, 'Send Mail (Outlook)', 'Sends a mail as the user on Outlook', '/me/sendMail', 'POST', '{"Authorization": "Bearer {token}", "Content-Type": "application/json"}', '{}', 'body');