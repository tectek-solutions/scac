-- Your SQL goes here
INSERT INTO actions (
        api_id,
        name,
        description,
        http_method,
        http_endpoint,
        http_parameters,
        http_headers,
        http_body,
        trigger_date_json_path,
        trigger_date_format
    )
VALUES (
        (
            SELECT id
            FROM apis
            WHERE name = 'Gmail'
        ),
        'New Mail Received (Gmail)',
        'Checks if a new mail is received on Gmail',
        'GET',
        '/gmail/v1/users/me/messages',
        '{"maxResults": 1}',
        '{"Authorization": "Bearer {token}"}',
        '{}',
        'messages[0].internalDate',
        '%Y-%m-%d %H:%M:%S'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Outlook'
        ),
        'New Mail Received (Outlook)',
        'Checks if a new mail is received on Outlook',
        'GET',
        '/me/messages',
        '{"$top": 1, "$select": "receivedDateTime"}',
        '{"Authorization": "Bearer {token}"}',
        '{}',
        'value[0].receivedDateTime',
        '%Y-%m-%dT%H:%M:%S.%fZ'
    );