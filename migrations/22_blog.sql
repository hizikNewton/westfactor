-- Ensure extensions are created if they don't exist
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Create or replace the function to generate passwords
CREATE OR REPLACE FUNCTION generate_password(length INT) RETURNS TEXT AS $$
DECLARE
    password TEXT;
BEGIN
    password := encode(gen_random_bytes(length), 'base64');
    password := regexp_replace(password, '[^a-zA-Z0-9]', '', 'g');
    password := substring(password FROM 1 FOR length);
    RETURN password;
END;
$$ LANGUAGE plpgsql;

-- Alter the users table
ALTER TABLE users
    ALTER COLUMN user_id SET DATA TYPE UUID USING user_id::UUID,
    ALTER COLUMN user_id SET DEFAULT uuid_generate_v4(),
    ALTER COLUMN password SET DATA TYPE VARCHAR(25),  -- Adjusted to VARCHAR(25) as per your script
    ALTER COLUMN password SET DEFAULT generate_password(12),  -- Adjust the length as needed
    ALTER COLUMN password SET NOT NULL;

-- Alter the categories table
ALTER TABLE categories
    ALTER COLUMN user_id SET DATA TYPE UUID USING user_id::UUID,
    DROP CONSTRAINT fk_user,  -- Remove the old constraint
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (user_id) ON UPDATE CASCADE ON DELETE CASCADE;


-- Alter the subcategories table
ALTER TABLE subcategories
    ALTER COLUMN user_id SET DATA TYPE UUID USING user_id::UUID,
    ALTER COLUMN user_id SET NOT NULL;
  
-- Alter the posts table
ALTER TABLE posts
    ALTER COLUMN user_id SET DATA TYPE UUID USING user_id::UUID,
    ALTER COLUMN user_id SET NOT NULL;
 