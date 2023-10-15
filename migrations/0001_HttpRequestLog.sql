CREATE TABLE IF NOT EXISTS http_request (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    sender_ip CIDR NOT NULL,
    verb VARCHAR (7) NOT NULL,
    target VARCHAR (255) NOT NULL,
    version CHAR(3) NOT NULL
);

CREATE TABLE IF NOT EXISTS headers (
    id  INTEGER NOT NULL,
    name VARCHAR (255) NOT NULL,
    value VARCHAR (255) NOT NULL,
    PRIMARY KEY (id, name),
    FOREIGN KEY (id)
        REFERENCES http_request (id)
);

CREATE TABLE IF NOT EXISTS body (
    id INTEGER NOT NULL,
    filename VARCHAR (255),
    body_location VARCHAR (255), /* For files or large texts */
    body_text VARCHAR (255), /* For small bodies containing just text */
    PRIMARY KEY (id),
    FOREIGN KEY (id)
        REFERENCES http_request (id)
);


