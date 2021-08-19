CREATE TABLE votings (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    date_from VARCHAR NOT NULL,
    date_to VARCHAR NOT NULL,
    description TEXT NOT NULL,
    details VARCHAR NOT NULL,
    proposer VARCHAR NOT NULL,
    forum_link VARCHAR NOT NULL,
    active BOOLEAN DEFAULT true
);