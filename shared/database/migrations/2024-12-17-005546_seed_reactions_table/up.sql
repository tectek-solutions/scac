-- Your SQL goes here
INSERT INTO reactions (
        apis_id,
        name,
        description,
        http_method,
        http_endpoint,
        http_parameters,
        http_headers,
        http_body
    )
VALUES (
        (
            SELECT id
            FROM apis
            WHERE name = 'Gmail'
        ),
        'Send Mail (Gmail)',
        'Sends a mail as the user on Gmail',
        'POST',
        '/gmail/v1/users/me/messages/send',
        '{}',
        '{"Authorization": "Bearer {token}"}',
        '{}'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Outlook'
        ),
        'Send Mail (Outlook)',
        'Sends a mail as the user on Outlook',
        'POST',
        '/me/sendMail',
        '{}',
        '{"Authorization": "Bearer {token}", "Content-Type": "application/json"}',
        '{}'
    );