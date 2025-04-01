CREATE TABLE users (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username VARCHAR(32) NOT NULL UNIQUE, -- 32 characters
    password_hash VARCHAR(60) NOT NULL, -- sha512 encrypt the password and then bcrypt that
    email VARCHAR(255) NOT NULL UNIQUE, -- theoretically there's no limit on email length, but..

    servers INT[] NOT NULL, -- see comment in servers table

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE servers (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(72) NOT NULL UNIQUE, -- 72 characters
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE, -- owner of the server

    users INT[] NOT NULL, -- Can't do two users relations so it just has to be a
                          -- half-assed array of user ids


    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE channels (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    server_id INT NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    name VARCHAR(72) NOT NULL, -- 72 characters
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    type INT NOT NULL DEFAULT 1 -- type of channel, 1 = text

    UNIQUE (server_id, name) -- unique channel names per server
);

CREATE TABLE messages (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    author_id INT NOT NULL REFERENCES users(id) ON DELETE SET NULL,

    channel_id INT NOT NULL REFERENCES channels(id) ON DELETE CASCADE,

    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
