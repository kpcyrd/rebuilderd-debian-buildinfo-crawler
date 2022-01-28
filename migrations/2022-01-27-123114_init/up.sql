CREATE TABLE buildinfos (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    url VARCHAR NOT NULL,
    content VARCHAR NOT NULL
);

CREATE TABLE artifacts (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    file_name VARCHAR NOT NULL,
    buildinfo_id INTEGER NOT NULL,
    FOREIGN KEY(buildinfo_id) REFERENCES buildinfos(id) ON DELETE CASCADE
);

CREATE INDEX buildinfo_url_idx ON buildinfos(url);
CREATE INDEX artifacts_file_name_idx ON artifacts(file_name);
