-- Your SQL goes here
CREATE EXTENSION pgcrypto;

CREATE TABLE apis (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  respository TEXT,
  documentation TEXT,
  homepage TEXT,
  lastUpdated TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
  creator_id TEXT NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE tags (
    api_id UUID NOT NULL,
    tag TEXT NOT NULL,
    PRIMARY KEY(api_id, tag),
    CONSTRAINT api_fkey
      FOREIGN KEY(api_id) 
	    REFERENCES apis(id)
);