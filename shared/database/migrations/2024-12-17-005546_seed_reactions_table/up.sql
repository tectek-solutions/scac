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
VALUES 
(
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
        '{
            "Authorization": "Bearer { token }",
            "Content-Type": "application/json",
            "Accept": "application/json"
        }',
        '{
            "raw": "From: {{ from }}\\r\\nTo: {{ to }}\\r\\nSubject: {{ subject }}\\r\\n\\r\\n{{ body }}"
        }',
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
        '{
            "Authorization": "Bearer { token }",
            "Content-Type": "application/json",
            "Accept": "application/json"
        }',
        '{"message":{"toRecipients":[{"emailAddress":{"address":"{{ to }}","name":" {{ to }}"}}],"subject":" {{ subject }}","body":{"content":"{{ body }}","contentType":"text"}}}',
        '{
            "to": "The recipient of the mail",
            "subject": "The subject of the mail",
            "body": "The body of the mail"
        }'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Spotify'
        ),
        'Play music (Spotify)',
        'Plays music on Spotify',
        'PUT',
        '/me/player/play',
        '{"device_id": "{ device_id }"}',
        '{
            "Authorization": "Bearer { token }",
            "Content-Type": "application/json",
            "Accept": "application/json"
        }',
        '{}',
        '{
            "device_id": "The device ID for the player"
        }'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Spotify'  
        ),
        'Pause music (Spotify)',
        'Pauses music on Spotify',
        'PUT',
        '/me/player/pause',
        '{"device_id": "{ device_id }"}',
        '{
            "Authorization": "Bearer { token }",
            "Content-Type": "application/json",
            "Accept": "application/json"
        }',
        '{}',
        '{
            "device_id": "The device ID for the player"
        }'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Spotify'
        ),
        'Play next song (Spotify)',
        'Plays the next song on Spotify',
        'POST',
        '/me/player/next',
        '{"device_id": "{ device_id }"}',
        '{
            "Authorization": "Bearer { token }",
            "Content-Type": "application/json",
            "Accept": "application/json"
        }',
        '{}',
        '{
            "device_id": "The device ID for the player"
        }'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Spotify'
        ),
        'Play previous song (Spotify)',
        'Plays the previous song on Spotify',
        'POST',
        '/me/player/previous',
        '{"device_id": "{ device_id }"}',
        '{
            "Authorization": "Bearer { token }",
            "Content-Type": "application/json",
            "Accept": "application/json"
        }',
        '{}',
        '{
            "device_id": "The device ID for the player"
        }'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Spotify'
        ),
        'Create playlist (Spotify)',
        'Creates a playlist on Spotify',
        'POST',
        '/me/playlists',
        '{}',
        '{
            "Authorization": "Bearer { token }",
            "Content-Type": "application/json",
            "Accept": "application/json"
        }',
        '{"name": "{{ name }}", "description": "{{ description }}", "public": "{{ public }}"}',
        '{
            "name": "The name of the playlist",
            "description": "The description of the playlist",
            "public": "The visibility of the playlist"
        }'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Github'
        ),
        'Create repository (Github)',
        'Creates a repository on Github',
        'POST',
        '/user/repos',
        '{}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
            "Content-Type": "application/json"
        }',
        '{"name": "{{ name }}", "description": "{{ description }}", "private": "{{ private }}"}',
        '{
            "name": "The name of the repository",
            "description": "The description of the repository",
            "private": "The visibility of the repository"
        }'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Github'
        ),
        'Create release (Github)',
        'Creates a release on Github',
        'POST',
        '/repos/{ owner }/{ repo }/releases',
        '{}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
            "Content-Type": "application/json"
        }',
        '{"tag_name": "{{ tag_name }}", "name": "{{ name }}"}',
        '{
            "owner": "The owner of the repository",
            "repo": "The repository name",
            "tag_name": "The tag name of the release",
            "name": "The name of the release"
        }'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Github'
        ),
        'Create issue (Github)',
        'Creates a issue on Github',
        'POST',
        '/repos/{ owner }/{ repo }/issues',
        '{}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
            "Content-Type": "application/json"
        }',
        '{"title": "{{ title }}", "body": "{{ body }}"}',
        '{
            "owner": "The owner of the repository",
            "repo": "The repository name",
            "title": "The title of the issue",
            "body": "The body of the issue"
        }'
    ),
    (
        (
            SELECT id
            FROM apis
            WHERE name = 'Github'
        ),
        'Add a collaborator (Github)',
        'Adds a collaborator to a repository on Github',
        'PUT',
        '/repos/{ owner }/{ repo }/collaborators/{ username }',
        '{}',
        '{
            "Authorization": "Bearer { token }",
            "Accept": "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
            "Content-Type": "application/json"
        }',
        '{}',
        '{
            "owner": "The owner of the repository",
            "repo": "The repository name",
            "username": "The username of the collaborator"
        }'
    );