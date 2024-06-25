-- Add migration script here
CREATE TABLE
    users (
        id serial primary key,
        user_id varchar(20) not null unique,
        first_name varchar(200) not null,
        last_name varchar(200) not null,
        email varchar(100) not null unique,
        password varchar(150) not null,
        isadmin boolean,
        time_stamp date default current_date
    );

CREATE TABLE
    categories (
        category_id serial ,
        category_name varchar(150) primary key,
        user_id varchar(20) not null,
        time_stamp date default current_date,
        CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (user_id)
    );

CREATE TABLE
    subcategories (
        subcategory_id serial,
        subcategory_name varchar(150) not null primary key,
        category_name varchar(150) not null,
        user_id varchar(20) not null,
        time_stamp date default current_date,
        CONSTRAINT fk_subcat2user FOREIGN KEY (user_id) REFERENCES users (user_id) ON UPDATE CASCADE ON DELETE NO ACTION,
        CONSTRAINT fk_subcat2cat FOREIGN KEY  (category_name) REFERENCES categories(category_name) ON UPDATE CASCADE ON DELETE CASCADE
    );

CREATE TABLE
    posts (
        id serial primary key,
        post_id varchar(20) not null unique,
        user_id varchar(20) not null,
        title varchar(255) not null,
        content varchar(2000) not null,
        views int not null default 0,
        likes int not null default 0,
        subcategory_name varchar(150) not null,
        CONSTRAINT fk_posts2userid FOREIGN KEY (user_id) REFERENCES users (user_id) ON UPDATE CASCADE ON DELETE NO ACTION,
        CONSTRAINT fk_post2subcat FOREIGN KEY (subcategory_name) REFERENCES subcategories (subcategory_name) ON UPDATE CASCADE ON DELETE CASCADE
    );