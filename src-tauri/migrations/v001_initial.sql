-- ============================================================
-- Migration v001: Initial schema for August Mark
-- ============================================================

-- Schema version tracking
CREATE TABLE IF NOT EXISTS schema_version (
    version     INTEGER PRIMARY KEY,
    applied_at  TEXT NOT NULL DEFAULT (datetime('now')),
    description TEXT
);

INSERT INTO schema_version (version, description)
VALUES (1, 'Initial schema');

-- ============================================================
-- Projects
-- ============================================================
CREATE TABLE projects (
    id          TEXT PRIMARY KEY,              -- UUID v4
    name        TEXT NOT NULL,
    description TEXT DEFAULT '',
    color       TEXT DEFAULT '#FF6B35',        -- Hex color for UI
    is_archived INTEGER NOT NULL DEFAULT 0,    -- 0 = active, 1 = archived
    is_deleted  INTEGER NOT NULL DEFAULT 0,    -- 0 = active, 1 = deleted
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Default project
INSERT INTO projects (id, name, description, color)
VALUES ('default', 'Default Project', 'Uncategorized reviews', '#FF6B35');

-- ============================================================
-- Sessions (a review session containing multiple captures)
-- ============================================================
CREATE TABLE sessions (
    id          TEXT PRIMARY KEY,              -- UUID v4
    project_id  TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title       TEXT NOT NULL,
    description TEXT DEFAULT '',
    status      TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed', 'archived')),
    is_deleted  INTEGER NOT NULL DEFAULT 0,    -- 0 = active, 1 = deleted
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT                          -- When session was ended
);

CREATE INDEX idx_sessions_project ON sessions(project_id);
CREATE INDEX idx_sessions_status ON sessions(status);
CREATE INDEX idx_sessions_created ON sessions(created_at DESC);

-- ============================================================
-- Captures (one overlay activation = one screenshot)
-- ============================================================
CREATE TABLE captures (
    id              TEXT PRIMARY KEY,          -- UUID v4
    session_id      TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    screenshot_path TEXT NOT NULL,             -- Relative path from app data dir
    monitor_name    TEXT,
    monitor_x       INTEGER,
    monitor_y       INTEGER,
    monitor_width   INTEGER,
    monitor_height  INTEGER,
    scale_factor    REAL DEFAULT 1.0,
    window_title    TEXT,                      -- Foreground window title at capture time
    is_deleted      INTEGER NOT NULL DEFAULT 0,    -- 0 = active, 1 = deleted
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_captures_session ON captures(session_id);

-- ============================================================
-- Issues (individual markers/annotations on a capture)
-- ============================================================
CREATE TABLE issues (
    id              TEXT PRIMARY KEY,          -- UUID v4
    capture_id      TEXT NOT NULL REFERENCES captures(id) ON DELETE CASCADE,
    session_id      TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE, -- Denormalized for fast query
    project_id      TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE, -- Denormalized for fast query

    -- Guide Step metadata
    marker_number   INTEGER NOT NULL,          -- ①②③ display number within capture
    title           TEXT NOT NULL,
    description     TEXT DEFAULT '',
    issue_type      TEXT NOT NULL DEFAULT 'Step',
    severity        TEXT NOT NULL DEFAULT 'Info',
    status          TEXT NOT NULL DEFAULT 'Open',

    -- Annotation position on screenshot (in screenshot pixels)
    marker_x        REAL NOT NULL,             -- Center X of marker
    marker_y        REAL NOT NULL,             -- Center Y of marker

    -- Annotation geometry (JSON string for flexibility)
    annotation_data TEXT NOT NULL DEFAULT '{}',

    -- Annotation style
    color           TEXT DEFAULT '#FF6B35',
    stroke_width    REAL DEFAULT 2.0,

    -- Crop image
    crop_path       TEXT,                      -- Relative path, NULL if not yet generated

    is_deleted      INTEGER NOT NULL DEFAULT 0,    -- 0 = active, 1 = deleted
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_issues_capture ON issues(capture_id);
CREATE INDEX idx_issues_session ON issues(session_id);
CREATE INDEX idx_issues_project ON issues(project_id);
CREATE INDEX idx_issues_type ON issues(issue_type);
CREATE INDEX idx_issues_severity ON issues(severity);
CREATE INDEX idx_issues_status ON issues(status);

-- ============================================================
-- Tags
-- ============================================================
CREATE TABLE tags (
    id      TEXT PRIMARY KEY,                  -- UUID v4
    name    TEXT NOT NULL UNIQUE,
    color   TEXT DEFAULT '#4ECDC4',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Many-to-many: issues ↔ tags
CREATE TABLE issue_tags (
    issue_id TEXT NOT NULL REFERENCES issues(id) ON DELETE CASCADE,
    tag_id   TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (issue_id, tag_id)
);

CREATE INDEX idx_issue_tags_tag ON issue_tags(tag_id);

-- ============================================================
-- Settings (key-value store)
-- ============================================================
CREATE TABLE settings (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL,
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Default settings
INSERT INTO settings (key, value) VALUES
    ('theme', '"dark"'),
    ('overlay_trigger', '"middle_mouse_hold"'),
    ('hold_duration_ms', '1000'),
    ('screenshot_quality', '90'),
    ('default_project_id', '"default"'),
    ('auto_backup', 'false'),
    ('gdrive_connected', 'false');

-- ============================================================
-- Triggers for cascading soft deletes (is_deleted = 1)
-- ============================================================
CREATE TRIGGER IF NOT EXISTS trg_soft_delete_project
AFTER UPDATE OF is_deleted ON projects
FOR EACH ROW
WHEN NEW.is_deleted = 1
BEGIN
    UPDATE sessions SET is_deleted = 1 WHERE project_id = OLD.id;
END;

CREATE TRIGGER IF NOT EXISTS trg_soft_delete_session
AFTER UPDATE OF is_deleted ON sessions
FOR EACH ROW
WHEN NEW.is_deleted = 1
BEGIN
    UPDATE captures SET is_deleted = 1 WHERE session_id = OLD.id;
END;

CREATE TRIGGER IF NOT EXISTS trg_soft_delete_capture
AFTER UPDATE OF is_deleted ON captures
FOR EACH ROW
WHEN NEW.is_deleted = 1
BEGIN
    UPDATE issues SET is_deleted = 1 WHERE capture_id = OLD.id;
END;

