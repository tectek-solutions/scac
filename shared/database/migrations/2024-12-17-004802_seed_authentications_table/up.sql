-- Your SQL goes here
INSERT INTO authentications (
        name,
        authentication_url,
        refresh_token_url,
        client_id,
        client_secret
    )
VALUES (
        'Google',
        'https://accounts.google.com/o/oauth2/v2/auth',
        'https://oauth2.googleapis.com/token',
        '936038757007-d2vvj4kjm98vcod9e9ek9ilvoeij1fcr.apps.googleusercontent.com',
        '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'
    ),
    (
        'Microsoft',
        'https://login.microsoftonline.com/common/oauth2/v2.0/authorize',
        'https://login.microsoftonline.com/common/oauth2/v2.0/token',
        '3e226b46-9ef1-42bf-a557-a73ca86aed7c',
        'jpK8Q~hhwAuZjrPU1t6IOVuEfZ5n6K6PaX0ptaiX'
    );