CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    "password" TEXT NOT NULL
);

-- ghosty:specterpoint
INSERT INTO users (username, password) VALUES ('ghosty', '$argon2id$v=19$m=19456,t=2,p=1$KJhJOgKpZMHtVDIB8S8M1w$bMT2sVQdkldcX8rIR5V1CfzCi9NkNR+FOY+tdHhyXL4');

CREATE TABLE IF NOT EXISTS listeners (
    id INTEGER PRIMARY KEY,
    "name" TEXT NOT NULL,
    "host" TEXT NOT NULL,
    port INTEGER NOT NULL,
    "type" TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS endpoints (
    id INTEGER PRIMARY KEY,
    listener_id INTEGER NOT NULL,
    endpoint TEXT NOT NULL,
    FOREIGN KEY (listener_id) REFERENCES listeners (id)
);

