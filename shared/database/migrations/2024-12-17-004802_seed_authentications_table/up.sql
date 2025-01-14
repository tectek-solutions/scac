-- Your SQL goes here
INSERT INTO authentications (
        name,
        authorization_url,
        authorization_http_parameters
        token_url,
        token_url_http_parameters,
        client_id,
        client_secret
    )
VALUES (
        'Google',
        'https://accounts.google.com/o/oauth2/v2/auth?client_id={ client_id }&redirect_uri={ redirect_uri }&state={ state }',
        '{
            response_type: "code",
            "scope": "https://mail.google.com/"
        }',
        'https://oauth2.googleapis.com/token',
        '{}',
        '936038757007-d2vvj4kjm98vcod9e9ek9ilvoeij1fcr.apps.googleusercontent.com',
        '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'
    ),
    (
        'Microsoft',
        'https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id={ client_id }&redirect_uri={ redirect_uri }&state={ state }',
        '{
            "response_type": "code",
            "scope": "email openid profile offline_access User.Read Mail.Read Mail.ReadWrite Mail.Send"
        }',
        'https://login.microsoftonline.com/common/oauth2/v2.0/token',
        '{}',
        '3e226b46-9ef1-42bf-a557-a73ca86aed7c',
        'jpK8Q~hhwAuZjrPU1t6IOVuEfZ5n6K6PaX0ptaiX'
    ),
    (
        'Spotify',
        'https://accounts.spotify.com/authorize?client_id={ client_id }&redirect_uri={ redirect_uri }&state={ state}',
        '{
            "response_type": "code",
            "scope": "user-read-private"
        }',
        'https://accounts.spotify.com/api/token',
        '{}',
        '03e70d87d1194dc5a9dbaeb69717dbf7',
        'b30fee01dd0e4388b19386bd6c9e6423'
    ),
    (
        'Reddit',
        'https://www.reddit.com/api/v1/authorize?client_id={ client_id }&redirect_uri={ redirect_uri }&state={ state }',
        '{
            "response_type": "code",
            "duration": "permanent",
            "scope": "identity"
        }',
        'https://www.reddit.com/api/v1/access_token',
        '{}',
        'o0pakhk3vj0ExwxHAGQNsw',
        'G6rQFY1oKJalTsyQTFFr71g47_6ofw'
    ),
    (
        'Twitter',
        'https://twitter.com/i/oauth2/authorize?client_id={ client_id }&redirect_uri={ redirect_uri }&state={ state }',
        '{
            "response_type": "code",
            "scope": "tweet.read users.read follows.read offline.access",
            "code_challenge": "challenge",
            "code_challenge_method": "plain"
        }',
        'https://api.x.com/2/oauth2/token',
        '{}',
        'TDBIUmgxeE5Qd3pfNmh2TlN3LTA6MTpjaQ',
        'lmYGXBmEhD89tnqZlSwCwiGAXSIWB_ct5YKh5fZt4-c8mZnMVD'
    ),
    (
        'Facebook',
        'https://www.facebook.com/v21.0/dialog/oauth?client_id={ client_id }&redirect_uri={ redirect_uri }&state={ state }',
        '{
            "response_type": "code",
            "scope": "public_profile"
        }',
        'https://graph.facebook.com/v21.0/oauth/access_token',
        '{}',
        '1296112668301301',
        '55f63103f67c32fe01a6fbbbe495e994'
    ),
    (
        'Github',
        'https://github.com/login/oauth/authorize?client_id={ client_id }&redirect_uri={ redirect_uri }&state={ state }',
        '{
            "response_type": "code",
            "scope": "user%20repo"
        }',
        'https://github.com/login/oauth/access_token',
        '{}',
        'Ov23li78H0Y0Sus4gKoe',
        '8da9212dd0c0dea2cb454aa80db0459f4a3fd835'
    ),
    (
        'SumUp',
        'https://api.sumup.com/authorize?response_type=code&client_id={ client_id }&redirect_uri={ redirect_uri }&state={ state }',
        '{
            "response_type": "code",
            "scope": "email%20profile"
        }',
        'https://api.sumup.com/token',
        '{}',
        'cc_classic_SUh58NKFbNqnhPjG5bS5hWrmzGY0E',
        'cc_sk_classic_XuIP1KOWmIiY6EZmtqFnq6cJ1amUhNk2wP6to5qGfM9PQH5gd5'
    )