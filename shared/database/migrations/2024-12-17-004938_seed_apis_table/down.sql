-- This file should undo anything in `up.sql`
DELETE FROM apis WHERE name = 'Gmail' OR name = 'Outllook';