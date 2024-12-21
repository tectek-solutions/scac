-- This file should undo anything in `up.sql`
DELETE FROM authentications WHERE name = 'Google' OR name = 'Microsoft';
