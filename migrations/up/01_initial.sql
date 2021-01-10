CREATE TABLE IF NOT EXISTS app_user (
    id SERIAL NOT NULL,
    username VARCHAR(31) NOT NULL UNIQUE,

    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS url_hash (
    hashval INT NOT NULL,
    url VARCHAR(255) NOT NULL,
    app_user_id INT NOT NULL,

    CONSTRAINT fk_app_user FOREIGN KEY(app_user_id) REFERENCES app_user(id) ON DELETE CASCADE,
    PRIMARY KEY(hashval)
);

CREATE TABLE IF NOT EXISTS hit (
    url_hash_hashval INT NOT NULL,
    ts TIMESTAMP WITH TIME ZONE NOT NULL,
    addr INET NOT NULL,

    CONSTRAINT fk_url_hash FOREIGN KEY(url_hash_hashval) REFERENCES url_hash(hashval) ON DELETE CASCADE,
    PRIMARY KEY(url_hash_hashval, ts, addr)
);