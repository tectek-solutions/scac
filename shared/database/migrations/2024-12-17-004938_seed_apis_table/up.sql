-- Your SQL goes here
INSERT INTO apis (authentications_id, name, base_url)
VALUES (
        (
            SELECT id
            FROM authentications
            WHERE name = 'Google'
        ),
        'Gmail',
        'https://gmail.googleapis.com'
    ),
    (
        (
            SELECT id
            FROM authentications
            WHERE name = 'Microsoft'
        ),
        'Outlook',
        'https://graph.microsoft.com'
    );