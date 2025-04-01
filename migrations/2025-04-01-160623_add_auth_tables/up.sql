CREATE TABLE user_auth_keys (
    hashed_key VARCHAR(60) GENERATED ALWAYS AS IDENTITY PRIMARY KEY --
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE
    scopes VARCHAR(60)[] NOT NULL -- scopes :D
);

CREATE TABLE user_auth_mfa (

);
-- not done
