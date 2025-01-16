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
        (SELECT id FROM apis WHERE name = 'Gmail'),
        'Send Mail (Gmail)',
        'Sends a mail as the user on Gmail',
        'POST',
        '/gmail/v1/users/me/messages/send',
        '{}',
        '{"Authorization": "Bearer {token}"}',
        '{"raw": "From: {from}\r\nTo: {to}\r\nSubject: {subject}\r\n\r\n{body}"}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Outlook'),
        'Send Mail (Outlook)',
        'Sends a mail as the user on Outlook',
        'POST',
        '/me/sendMail',
        '{}',
        '{"Authorization": "Bearer {token}", "Content-Type": "application/json"}',
        '{"message":{"toRecipients":[{"emailAddress":{"address":"{to}","name":" {to}"}}],"subject":" {subject}","body":{"content":"{body}","contentType":"text"}}}  '
    ),
    (
        (SELECT id FROM apis WHERE name = 'Spotify'),
        'Play music (Spotify)',
        'Plays music on Spotify',
        'PUT',
        '/me/player/play',
        '{"device_id": "{device_id}"}',
        '{"Authorization": "Bearer {token}", "Content-Type": "application/json"}',
        '{}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Spotify'),
        'Pause music (Spotify)',
        'Pauses music on Spotify',
        'PUT',
        '/me/player/pause',
        '{"device_id": "{device_id}"}',
        '{"Authorization": "Bearer {token}", "Content-Type": "application/json"}',
        '{}'
    ),
     (
        (SELECT id FROM apis WHERE name = 'Spotify'),
        'Play next song (Spotify)',
        'Plays the next song on Spotify',
        'POST',
        '/me/player/next',
        '{"device_id": "{device_id}"}',
        '{"Authorization": "Bearer {token}", "Content-Type": "application/json"}',
        '{}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Spotify'),
        'Play previous song (Spotify)',
        'Plays the previous song on Spotify',
        'POST',
        '/me/player/previous',
        '{"device_id": "{device_id}"}',
        '{"Authorization": "Bearer {token}", "Content-Type": "application/json"}',
        '{}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Spotify'),
        'Create playlist (Spotify)',
        'Creates a playlist on Spotify',
        'POST',
        '/me/playlists',
        '{}',
        '{"Authorization": "Bearer {token}", "Content-Type": "application/json"}',
        '{"name": "{name}", "description": "{description}", "public": {public}}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Github'),
        'Create repository (Github)',
        'Creates a repository on Github',
        'POST',
        '/user/repos',
        '{}',
        '{"Authorization": "Bearer {token}", "Accept": "application/vnd.github+json", "X-GitHub-Api-Version": "2022-11-28"}',
        '{"name": "{name}", "description": "{description}", "private": {private}}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Github'),
        'Create release (Github)',
        'Creates a release on Github',
        'POST',
        '/repos/{owner}/{repo}/releases',
        '{}',
        '{"Authorization": "Bearer {token}", "Accept": "application/vnd.github+json", "X-GitHub-Api-Version": "2022-11-28"}',
        '{"tag_name": "{tag_name}", "name": "{name}"}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Github'),
        'Create issue (Github)',
        'Creates a issue on Github',
        'POST',
        '/repos/{owner}/{repo}/issues',
        '{}',
        '{"Authorization": "Bearer {token}", "Accept": "application/vnd.github+json", "X-GitHub-Api-Version": "2022-11-28"}',
        '{"title": "{title}", "body": "{body}"}'
    ),
    (
        (SELECT id FROM apis WHERE name = 'Github'),
        'Add a collaborator (Github)',
        'Adds a collaborator to a repository on Github',
        'PUT',
        '/repos/{owner}/{repo}/collaborators/{username}',
        '{}',
        '{"Authorization": "Bearer {token}", "Accept": "application/vnd.github+json", "X-GitHub-Api-Version": "2022-11-28"}',
        '{}'
    ),
    ;