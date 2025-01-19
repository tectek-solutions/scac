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
        'New Mail Received (Outlook) -> Send Mail (Outlook)',
        'Checks if a new mail is received on Gmail and sends it on Outlook',
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
            "to": "xavierclementantoine@gmai.com",
            "subject": "Mail send from Outlook via SCAC",
            "body": "Hello, this mail is sent from Outlook via SCAC"
        }'
    ),