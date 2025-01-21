INSERT INTO actions (
        apis_id,
        name,
        description,
        http_method,
        http_endpoint,
        http_parameters,
        http_headers,
        http_body,
        data_keys,
        last_id_json_path
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
        '{}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/json"
        }',
        '{}',
        '{}',
        '/messages/0/id'
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
        '{"$select": "receivedDateTime"}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/json",
            "Content-Type": "application/json"
        }',
        '{}',
        '{}',
        '/value/0/id'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Spotify'
        ),
        'New Song Added (Spotify)',
        'Checks if a new song is added to the user''s library on Spotify',
        'GET',
        '/me/tracks',
        '{}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/json",
            "Content-Type": "application/json"
        }',
        '{}',
        '{}',
        '/items/0/track/id'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Spotify'
        ),
        'Currently Playing Song (Spotify)',
        'Checks if the user is currently playing a song on Spotify',
        'GET',
        '/me/player/currently-playing',
        '{}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/json",
            "Content-Type": "application/json"
        }',
        '{}',
        '{}',
        '/item/id'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Spotify'
        ),
        'New Playlist Created (Spotify)',
        'Checks if a new playlist is created on Spotify',
        'GET',
        '/me/playlists',
        '{}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/json",
            "Content-Type": "application/json"
        }',
        '{}',
        '{}',
        '/items/0/id'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Github'
        ),
        'New Repository Created (Github)',
        'Checks if a new repository is created on Github',
        'GET',
        '/user/repos',
        '{"sort": "created"}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/json",
            "Content-Type": "application/json"
        }',
        '{}',
        '{
            "owner": "The owner of the repository"
        }',
        '/'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Github'
        ),
        'New Commit Made (Github)',
        'Checks if a new commit is made on Github',
        'GET',
        '/repos/{owner}/{repo}/commits',
        '{}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/json",
            "Content-Type": "application/json"
        }',
        '{}',
        '{
            "owner": "The owner of the repository",
            "repo": "The repository name"
        }',
        '/'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Github'
        ),
        'New member added to repository (Github)',
        'Checks if a new member is added to a repository on Github',
        'GET',
        '/repos/{owner}/{repo}/collaborators',
        '{}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/json",
            "Content-Type": "application/json"
        }',
        '{}',
        '{
            "owner": "The owner of the repository",
            "repo": "The repository name"
        }',
        '/'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'SumUp'
        ),
        'New Transaction Made (SumUp)',
        'Checks if a new transaction is made on SumUp',
        'GET',
        '/v2.1/merchants/{merchant_code}/transactions/history',
        '{"order": "descending"}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/json",
            "Content-Type": "application/json"
        }',
        '{}',
        '{
            "merchant_code": "The merchant code"
        }',
        '/'
    );