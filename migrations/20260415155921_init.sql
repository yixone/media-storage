CREATE TABLE assets (
    id              BIGINT          NOT NULL,
    media           VARCHAR(64)     NOT NULL,

    created_at      TIMESTAMPTZ     NOT NULL,

    title           VARCHAR(256),
    caption         VARCHAR(1024),

    source_url      VARCHAR(512),

    PRIMARY KEY (id),
    FOREIGN KEY (media) REFERENCES media(id) ON DELETE CASCADE
);

CREATE TABLE media (
    id              VARCHAR(64)     NOT NULL,

    created_at      TIMESTAMPTZ     NOT NULL,

    blob_size       BIGINT          NOT NULL,
    content_type    VARCHAR(128)    NOT NULL,

    color           VARCHAR(7),

    width           INT,
    height          INT,

    status          VARCHAR(28)     NOT NULL    CHECK (
        status IN ('pending', 'processing', 'ready', 'failed')
    ),

    PRIMARY KEY (id)
);
