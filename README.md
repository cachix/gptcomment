# GPTComment

It scans your source code for a comment beginning with GPTComment: and executes the instruction that comes after it against the current file.

For example, if you have `schema.sql`:

```sql
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
```

Running `gptcomment schema.sql` will change it to:

```sql
CREATE TABLE houses (
    house_id SERIAL PRIMARY KEY,
    full_name VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    city VARCHAR(255) NOT NULL,
    state VARCHAR(255) NOT NULL,
    zip_code VARCHAR(255) NOT NULL,
    price INTEGER NOT NULL
);
```

It's meant to be used as a pre-commit hook to enforce code conventions and best practices.