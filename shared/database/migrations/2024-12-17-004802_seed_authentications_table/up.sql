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
        'https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={ client_id }&redirect_uri={ redirect_uri }&scope=https://mail.google.com/&state={ state}',
        'https://oauth2.googleapis.com/token',
        '936038757007-d2vvj4kjm98vcod9e9ek9ilvoeij1fcr.apps.googleusercontent.com',
        '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'
    ),
    (
        'Microsoft',
        'https://login.microsoftonline.com/common/oauth2/v2.0/authorize?response_type=code&client_id={ client_id }&redirect_uri={ redirect_uri }&scope=email openid profile offline_access User.Read Mail.Read Mail.ReadWrite Mail.Send&state={ state}',
        'https://login.microsoftonline.com/common/oauth2/v2.0/token',
        '3e226b46-9ef1-42bf-a557-a73ca86aed7c',
        'jpK8Q~hhwAuZjrPU1t6IOVuEfZ5n6K6PaX0ptaiX'
    ),
    (
        'Discord',
        'https://discord.com/oauth2/authorize?client_id={ client_id }&response_type=code&redirect_uri={ redirect_uri }&scope=dm_channels.messages.write dm_channels.read dm_channels.messages.read',
        'https://discord.com/api/oauth2/token',
        '1326172743338561667',
        '52VJGLl-l4iwxeHTjOuPhcse10Mmsi1E'
    )