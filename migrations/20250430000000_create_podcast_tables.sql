-- Create podcast-related tables
CREATE TABLE IF NOT EXISTS podcast_request_table (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(50) NOT NULL,
    title VARCHAR(5000) NOT NULL,
    author VARCHAR(5000) NOT NULL,
    rssUrl VARCHAR(5000) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS podcast_channels (
    id SERIAL PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    link VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    copyright VARCHAR(500),
    language VARCHAR(50) NOT NULL,
    author VARCHAR(255) NOT NULL,
    owner_email VARCHAR(255) NOT NULL,
    owner_name VARCHAR(255) NOT NULL,
    image_url VARCHAR(500) NOT NULL,
    last_build_date TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS podcast_episodes (
    id SERIAL PRIMARY KEY,
    channel_id INTEGER NOT NULL REFERENCES podcast_channels(id),
    guid VARCHAR(500) NOT NULL,
    title VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    link VARCHAR(500) NOT NULL,
    pub_date TIMESTAMPTZ NOT NULL,
    duration INTEGER NOT NULL,
    explicit BOOLEAN NOT NULL,
    image_url VARCHAR(500),
    media_url VARCHAR(500) NOT NULL,
    media_type VARCHAR(100) NOT NULL,
    media_length BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS podcast_channel_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS podcast_episode_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS channel_category_map (
    channel_id INTEGER NOT NULL REFERENCES podcast_channels(id),
    category_id INTEGER NOT NULL REFERENCES podcast_channel_categories(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (channel_id, category_id)
);

CREATE TABLE IF NOT EXISTS episode_category_map (
    episode_id INTEGER NOT NULL REFERENCES podcast_episodes(id),
    category_id INTEGER NOT NULL REFERENCES podcast_episode_categories(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (episode_id, category_id)
);

-- Create indexes for podcast tables (these improve query performance)
CREATE INDEX IF NOT EXISTS idx_podcast_request_uuid ON podcast_request_table(uuid);
CREATE INDEX IF NOT EXISTS idx_podcast_episodes_channel ON podcast_episodes(channel_id);
CREATE INDEX IF NOT EXISTS idx_podcast_episodes_guid ON podcast_episodes(guid);
CREATE INDEX IF NOT EXISTS idx_podcast_episodes_pubdate ON podcast_episodes(pub_date);
CREATE INDEX IF NOT EXISTS idx_channel_category_map_channel ON channel_category_map(channel_id);
CREATE INDEX IF NOT EXISTS idx_channel_category_map_category ON channel_category_map(category_id);
CREATE INDEX IF NOT EXISTS idx_episode_category_map_episode ON episode_category_map(episode_id);
CREATE INDEX IF NOT EXISTS idx_episode_category_map_category ON episode_category_map(category_id);