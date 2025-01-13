-- Your SQL goes here
INSERT INTO user_tokens (
        users_id,
        authentications_id,
        access_token,
        refresh_token
    )
VALUES (
        (
            SELECT id
            FROM users
            WHERE email = 'clement-antoine.xavier@epitech.eu'
        ),
        (
            SELECT id
            FROM authentications
            WHERE name = 'Google'
        ),
        '',
        ''
    );