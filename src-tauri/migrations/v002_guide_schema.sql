-- ============================================================
-- Migration v002: Guide Schema Update for August Guide
-- ============================================================

-- Drop triggers that reference the table before rebuilding
DROP TRIGGER IF EXISTS trg_soft_delete_capture;

CREATE TABLE IF NOT EXISTS issues_new (
    id              TEXT PRIMARY KEY,
    capture_id      TEXT NOT NULL REFERENCES captures(id) ON DELETE CASCADE,
    session_id      TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    project_id      TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,

    marker_number   INTEGER NOT NULL,
    title           TEXT NOT NULL,
    description     TEXT DEFAULT '',
    issue_type      TEXT NOT NULL DEFAULT 'Step',
    severity        TEXT NOT NULL DEFAULT 'Info',
    status          TEXT NOT NULL DEFAULT 'Open',

    marker_x        REAL NOT NULL,
    marker_y        REAL NOT NULL,

    annotation_data TEXT NOT NULL DEFAULT '{}',

    color           TEXT DEFAULT '#FF6B35',
    stroke_width    REAL DEFAULT 2.0,

    crop_path       TEXT,

    is_deleted      INTEGER NOT NULL DEFAULT 0,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

INSERT INTO issues_new (
    id, capture_id, session_id, project_id,
    marker_number, title, description, issue_type, severity, status,
    marker_x, marker_y, annotation_data, color, stroke_width,
    crop_path, is_deleted, created_at, updated_at
)
SELECT 
    id, capture_id, session_id, project_id,
    marker_number, title, description,
    CASE 
        WHEN issue_type IN ('Bug', 'UI', 'UX', 'Requirement', 'Suggestion', 'Question') THEN 'Step'
        ELSE issue_type 
    END,
    severity, status,
    marker_x, marker_y,
    annotation_data, color, stroke_width,
    crop_path, is_deleted, created_at, updated_at
FROM issues;

DROP TABLE IF EXISTS issues;
ALTER TABLE issues_new RENAME TO issues;

CREATE INDEX IF NOT EXISTS idx_issues_capture ON issues(capture_id);
CREATE INDEX IF NOT EXISTS idx_issues_session ON issues(session_id);
CREATE INDEX IF NOT EXISTS idx_issues_project ON issues(project_id);
CREATE INDEX IF NOT EXISTS idx_issues_type ON issues(issue_type);
CREATE INDEX IF NOT EXISTS idx_issues_severity ON issues(severity);
CREATE INDEX IF NOT EXISTS idx_issues_status ON issues(status);

-- Recreate trigger for soft delete cascade
CREATE TRIGGER IF NOT EXISTS trg_soft_delete_capture
AFTER UPDATE OF is_deleted ON captures
FOR EACH ROW
WHEN NEW.is_deleted = 1
BEGIN
    UPDATE issues SET is_deleted = 1 WHERE capture_id = OLD.id;
END;

-- Update schema version
INSERT INTO schema_version (version, description)
VALUES (2, 'Remove restrictive CHECK constraints and migrate to GuideStep schema');
