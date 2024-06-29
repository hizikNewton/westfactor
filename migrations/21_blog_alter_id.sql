-- Altering the users table
ALTER TABLE users 
ALTER COLUMN user_id TYPE varchar(200),
ALTER COLUMN user_id SET NOT NULL,
ALTER COLUMN time_stamp TYPE TIMESTAMPTZ,
ALTER COLUMN time_stamp SET DEFAULT NOW();

-- Altering the categories table
ALTER TABLE categories 
ALTER COLUMN user_id TYPE varchar(200),
ALTER COLUMN user_id SET NOT NULL,
ALTER COLUMN time_stamp TYPE TIMESTAMPTZ,
ALTER COLUMN time_stamp SET DEFAULT NOW();

-- Altering the subcategories table
ALTER TABLE subcategories 
ALTER COLUMN user_id TYPE varchar(200),
ALTER COLUMN user_id SET NOT NULL,
ALTER COLUMN time_stamp TYPE TIMESTAMPTZ,
ALTER COLUMN time_stamp SET DEFAULT NOW();

-- Altering the posts table
ALTER TABLE posts 
ALTER COLUMN user_id TYPE varchar(200),
ALTER COLUMN user_id SET NOT NULL,
ADD COLUMN time_stamp TIMESTAMPTZ DEFAULT NOW();