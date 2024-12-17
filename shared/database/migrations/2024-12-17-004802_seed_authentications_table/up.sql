-- Your SQL goes here
INSERT INTO authentications (
        name,
        authentication_url,
        refresh_token_url,
        access_token_json_path,
        refresh_token_json_path,
        access_token_expires_at_json_path,
        refresh_token_expires_at_json_path,
        is_expires_at_relative,
        client_id,
        client_secret
    )
VALUES (
        'Google',
        'https://accounts.google.com/o/oauth2/v2/auth',
        'https://oauth2.googleapis.com/token',
        'access_token',
        'refresh_token',
        'expires_in',
        'expires_in',
        TRUE,
        '936038757007-d2vvj4kjm98vcod9e9ek9ilvoeij1fcr.apps.googleusercontent.com',
        ''
    ),
    (
        'Microsoft',
        'https://login.microsoftonline.com/common/oauth2/v2.0/authorize',
        'https://login.microsoftonline.com/common/oauth2/v2.0/token',
        'access_token',
        'refresh_token',
        'expires_in',
        'expires_in',
        TRUE,
        '3e226b46-9ef1-42bf-a557-a73ca86aed7c',
        ''
    );