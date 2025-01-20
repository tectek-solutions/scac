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
        'https://graph.microsoft.com/v1.0'
    ),
    (
        (
            SELECT id
            FROM authentications
            WHERE name = 'Spotify'
        ),
        'Spotify',
        'https://api.spotify.com/v1'
    ),
    (
        (
            SELECT id
            FROM authentications
            WHERE name = 'Reddit'
        ),
        'Reddit',
        'https://oauth.reddit.com'
    ),  
    (
        (
            SELECT id
            FROM authentications
            WHERE name = 'Facebook'
        ),
        'Facebook',
        'https://graph.facebook.com'
    ),
    (
        (
            SELECT id
            FROM authentications
            WHERE name = 'Github'
        ),
        'Github',
        'https://api.github.com'
    ),
    (
        (
            SELECT id
            FROM authentications
            WHERE name = 'SumUp'
        ),
        'SumUp',
        'https://api.sumup.com/v0.1'
    );