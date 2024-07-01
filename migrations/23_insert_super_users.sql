ALTER TABLE users
    ADD COLUMN is_super_admin BOOLEAN DEFAULT false NOT NULL;

INSERT INTO users (first_name, last_name,email,is_super_admin,isadmin) VALUES ('Evelyn','Williams','tumeewes2015@gmail.com',true,true);

INSERT INTO users (first_name, last_name,email,is_super_admin,isadmin) VALUES ('Isaac','Bamidele','bamideleisaac11@gmail.com',true,true);