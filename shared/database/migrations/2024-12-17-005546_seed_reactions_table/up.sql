-- Your SQL goes here
INSERT INTO reactions (
        apis_id,
        name,
        description,
        http_method,
        http_endpoint,
        http_parameters,
        http_headers,
        http_body,
        data_keys
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
        '{"raw": "From: {from}\r\nTo: {to}\r\nSubject: {subject}\r\n\r\n{body}"}',
        '{
            "from": "The sender of the mail",
            "to": "The recipient of the mail",
            "subject": "The subject of the mail",
            "body": "The body of the mail"
        }'
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
        '{"message":{"toRecipients":[{"emailAddress":{"address":"{ to }","name":" { to }"}}],"subject":" { subject }","body":{"content":"{ body }","contentType":"text"}}}',
        '{
            "to": "The recipient of the mail",
            "subject": "The subject of the mail",
            "body": "The body of the mail"
        }'
    );