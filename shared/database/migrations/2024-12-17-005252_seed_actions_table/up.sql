INSERT INTO actions (
    apis_id,
    name,
    description,
    http_method,
    http_endpoint,
    http_parameters,
    http_headers,
    http_body
)
VALUES
    (
        (SELECT id FROM apis WHERE name = 'Gmail'),
        'New Mail Received (Gmail)',
        'Checks if a new mail is received on Gmail',
        'GET',
        '/gmail/v1/users/me/messages',
        '{"maxResults": 1}',
        '{
            "Authorization": "Bearer {token}",
            "Accept": "application/json"
            "Content-Type": "application/json"
        }',
        '{}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Outlook'),
        'New Mail Received (Outlook)',
        'Checks if a new mail is received on Outlook',
        'GET',
        '/me/messages',
        '{"$top": 1, "$select": "receivedDateTime"}',
        '{"Authorization": "Bearer {token}"}',
        '{}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Spotify'),
        'New Song Added (Spotify)',
        'Checks if a new song is added to the user''s library on Spotify',
        'GET',
        '/me/tracks',
        '{"limit": 1}',
        '{"Authorization": "Bearer {token}"}',
        '{}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Github'),
        'New Repository Created (Github)',
        'Checks if a new repository is created on Github',
        'GET',
        '/users/{owner}/repos',
        '{"sort": "created", "per_page": 1}',
        '{"Authorization": "Bearer {token}"}',
        '{}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Github'),
        'New Commit Made (Github)',
        'Checks if a new commit is made on Github',
        'GET',
        '/repos/{owner}/{repo}/commits',
        '{"per_page": 1}',
        '{"Authorization": "Bearer {token}"}',
        '{}'
    );
