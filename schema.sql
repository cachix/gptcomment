-- GPTComment: download https://launchbylunch.com/posts/2014/Feb/16/sql-naming-conventions/ and apply conventions

CREATE TABLE house (
    id SERIAL PRIMARY KEY,
    fullName VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    city VARCHAR(255) NOT NULL,
    state VARCHAR(255) NOT NULL,
    zip VARCHAR(255) NOT NULL,
    price INTEGER NOT NULL
);

CREATE TABLE agent (
    id SERIAL PRIMARY KEY,
    fullName VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL
    house INTEGER REFERENCES house(id)
);