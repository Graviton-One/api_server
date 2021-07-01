CREATE TABLE uni_stats 
    (id SERIAL PRIMARY KEY,
     tvl INT NULL,
     volume INT NULL, 
     addresses_count INT NULL,
     apy INT NULL,
     date TIMESTAMP NOT NULL DEFAULT NOW());

CREATE TABLE dodo_stats 
    (id SERIAL PRIMARY KEY,
     tvl INT NULL,
     volume INT NULL, 
     addresses_count INT NULL,
     apy INT NULL,
     date TIMESTAMP NOT NULL DEFAULT NOW());

CREATE TABLE sushi_stats 
    (id SERIAL PRIMARY KEY,
     tvl INT NULL,
     volume INT NULL, 
     addresses_count INT NULL,
     apy INT NULL,
     date TIMESTAMP NOT NULL DEFAULT NOW());

CREATE TABLE spooky_ftm_stats 
    (id SERIAL PRIMARY KEY,
     tvl INT NULL,
     volume INT NULL, 
     addresses_count INT NULL,
     apy INT NULL,
     date TIMESTAMP NOT NULL DEFAULT NOW());
     
CREATE TABLE spooky_usdc_stats 
    (id SERIAL PRIMARY KEY,
     tvl INT NULL,
     volume INT NULL, 
     addresses_count INT NULL,
     apy INT NULL,
     date TIMESTAMP NOT NULL DEFAULT NOW());

CREATE TABLE spirit_ftm_stats 
    (id SERIAL PRIMARY KEY,
     tvl INT NULL,
     volume INT NULL, 
     addresses_count INT NULL,
     apy INT NULL,
     date TIMESTAMP NOT NULL DEFAULT NOW());

CREATE TABLE spirit_usdc_stats 
    (id SERIAL PRIMARY KEY,
     tvl INT NULL,
     volume INT NULL, 
     addresses_count INT NULL,
     apy INT NULL,
     date TIMESTAMP NOT NULL DEFAULT NOW());

CREATE TABLE spirit_fusdt_stats 
    (id SERIAL PRIMARY KEY,
     tvl INT NULL,
     volume INT NULL, 
     addresses_count INT NULL,
     apy INT NULL,
     date TIMESTAMP NOT NULL DEFAULT NOW());

CREATE TABLE pancake_busd_stats
    (id SERIAL PRIMARY KEY,
     tvl INT NULL,
     volume INT NULL, 
     addresses_count INT NULL,
     apy INT NULL,
     date TIMESTAMP NOT NULL DEFAULT NOW());

CREATE TABLE pancake_bnb_stats
    (id SERIAL PRIMARY KEY,
     tvl INT NULL,
     volume INT NULL, 
     addresses_count INT NULL,
     apy INT NULL,
     date TIMESTAMP NOT NULL DEFAULT NOW());