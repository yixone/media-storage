CREATE TABLE assets (
    id          UUID            NOT NULL,
    media       VARCHAR(64)     NOT NULL,
    created_at  TIMESTAMPTZ     NOT NULL,
    title       VARCHAR(256),
    caption     VARCHAR(1024),
    source_url  VARCHAR(512),

    PRIMARY KEY (id),
    FOREIGN KEY (media) REFERENCES media(id) ON DELETE CASCADE
);

CREATE TABLE media (
    id          VARCHAR(64)     NOT NULL,
    state       VARCHAR(28)     NOT NULL    CHECK (
        state IN ('pending', 'processing', 'ready', 'failed')
    ),
    created_at  TIMESTAMPTZ     NOT NULL,
    size        BIGINT          NOT NULL,
    mimetype    VARCHAR(128)    NOT NULL,
    width       INT,
    height      INT,
    color       VARCHAR(6),

    PRIMARY KEY (id)
);
