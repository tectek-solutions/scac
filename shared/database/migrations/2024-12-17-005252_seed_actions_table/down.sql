-- This file should undo anything in `up.sql`
DELETE FROM actions WHERE name = 'New Mail Received (Gmail)' OR name = 'New Mail Received (Outlook)';