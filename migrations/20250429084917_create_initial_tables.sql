-- Add migration script here
-- Basic tables
CREATE TABLE IF NOT EXISTS users (
    uuid VARCHAR(50) PRIMARY KEY,
    timestamp VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS votes (
    uuid VARCHAR(50) NOT NULL,
    sessionId VARCHAR(50) NOT NULL,
    rating INTEGER NOT NULL,
    timestamp VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (uuid, sessionId)
);

CREATE TABLE IF NOT EXISTS feedback (
    uuid VARCHAR(50) NOT NULL,
    sessionId VARCHAR(50) NOT NULL,
    feedback VARCHAR(5000) NOT NULL,
    timestamp VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (uuid, sessionId)
);

-- Conference tables
CREATE TABLE IF NOT EXISTS conference_sessions (
    id VARCHAR(50) PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    room_id INTEGER,
    is_service_session BOOLEAN DEFAULT FALSE,
    is_plenum_session BOOLEAN DEFAULT FALSE,
    status VARCHAR(50) DEFAULT 'draft',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT starts_before_ends CHECK (starts_at <= ends_at)
);

CREATE TABLE IF NOT EXISTS conference_speakers (
    id VARCHAR(50) PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    bio VARCHAR(5000),
    tag_line VARCHAR(500),
    profile_picture VARCHAR(500),
    is_top_speaker BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS conference_rooms (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    sort INTEGER,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS conference_categories (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    sort INTEGER,
    type VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Relationship tables
CREATE TABLE IF NOT EXISTS session_speakers (
    session_id VARCHAR(50) NOT NULL REFERENCES conference_sessions(id),
    speaker_id VARCHAR(50) NOT NULL REFERENCES conference_speakers(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (session_id, speaker_id)
);

CREATE TABLE IF NOT EXISTS session_categories (
    session_id VARCHAR(50) NOT NULL REFERENCES conference_sessions(id),
    category_item_id INTEGER NOT NULL REFERENCES conference_categories(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (session_id, category_item_id)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_users_uuid ON users(uuid);
CREATE INDEX IF NOT EXISTS idx_votes_uuid ON votes(uuid);
CREATE INDEX IF NOT EXISTS idx_votes_sessionid ON votes(sessionId);
CREATE INDEX IF NOT EXISTS idx_feedback_uuid ON feedback(uuid);
CREATE INDEX IF NOT EXISTS idx_feedback_sessionid ON feedback(sessionId);
CREATE INDEX IF NOT EXISTS idx_sessions_id ON conference_sessions(id);
CREATE INDEX IF NOT EXISTS idx_speakers_id ON conference_speakers(id);
CREATE INDEX IF NOT EXISTS idx_session_dates ON conference_sessions(starts_at, ends_at);
CREATE INDEX IF NOT EXISTS idx_session_speakers_session ON session_speakers(session_id);
CREATE INDEX IF NOT EXISTS idx_session_speakers_speaker ON session_speakers(speaker_id);
CREATE INDEX IF NOT EXISTS idx_session_categories_session ON session_categories(session_id);
CREATE INDEX IF NOT EXISTS idx_session_categories_category ON session_categories(category_item_id);