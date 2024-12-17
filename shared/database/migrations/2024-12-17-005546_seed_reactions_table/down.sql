-- This file should undo anything in `up.sql`
DELETE FROM reactions WHERE name = 'Send Mail (Gmail)' OR name = 'Send Mail (Outlook)';