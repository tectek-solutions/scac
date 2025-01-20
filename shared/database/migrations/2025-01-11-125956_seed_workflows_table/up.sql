-- Your SQL goes here
INSERT INTO workflows (
        users_id,
        name,
        description,
        actions_id,
        reactions_id,
        action_data,
        reaction_data
    )
VALUES (
        (
            SELECT id
            FROM users
            WHERE email = 'clement-antoine.xavier@epitech.eu'
        ),
        'Send a mail with Gmail when a new mail is received on Gmail',
        'This workflow sends a mail with Gmail when a new mail is received on Gmail',
        (
            SELECT id
            FROM actions
            WHERE name = 'New Mail Received (Gmail)'
        ),
        (
            SELECT id
            FROM reactions
            WHERE name = 'Send Mail (Gmail)'
        ),
        '{}',
        '{
            "from": "xavierclementantoineà@gmail.com",
            "to": "mathieu974440@gmail.com",
            "subject": "New mail for Mathieu",
            "body": "A new mail has been received on Gmail"
        }'
    ),
    (
        (
            SELECT id
            FROM users
            WHERE email = 'clement-antoine.xavier@epitech.eu'
        ),
        'Send a mail with Outlook when a new mail is received on Outlook',
        'This workflow sends a mail with Outlook when a new mail is received on Outlook',
        (
            SELECT id
            FROM actions
            WHERE name = 'New Mail Received (Outlook)'
        ),
        (
            SELECT id
            FROM reactions
            WHERE name = 'Send Mail (Outlook)'
        ),
        '{}',
        '{
            "to": "mathieu.rasoanaivo@epitech.eu",
            "subject": "New email for Mathieu",
            "body": "A new email has been received on Outlook"
        }'
    ),
    (
        (
            SELECT id
            FROM users
            WHERE email = 'clement-antoine.xavier@epitech.eu'
        ),
        'When a new song is added to the user''s library on Spotify, play it on Spotify',
        'This workflow plays a new song added to the user''s library on Spotify',
        (
            SELECT id
            FROM actions
            WHERE name = 'New Song Added (Spotify)'
        ),
        (
            SELECT id
            FROM reactions
            WHERE name = 'Play music (Spotify)'
        ),
        '{}',
        '{
            "device_id": ""
        }'
    );