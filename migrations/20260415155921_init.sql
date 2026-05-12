CREATE TABLE assets (
    id              BIGINT          NOT NULL,
    media_key       VARCHAR(64)     NOT NULL,

    created_at      TIMESTAMPTZ     NOT NULL,

    title           VARCHAR(256),
    caption         VARCHAR(2048),

    source_url      VARCHAR(1024),

    deleted_at      TIMESTAMPTZ,

    PRIMARY KEY (id),
    FOREIGN KEY (media_key) REFERENCES media(id) ON DELETE CASCADE
);

CREATE TABLE media (
    id              VARCHAR(64)     NOT NULL,
    preview_key     VARCHAR(64)     NOT NULL,

    created_at      TIMESTAMPTZ     NOT NULL,

    blob_size       BIGINT          NOT NULL,
    preview_size    INT             NOT NULL,

    content_type    VARCHAR(128)    NOT NULL,

    accent_color    VARCHAR(7),

    width           INT,
    height          INT,

    status          VARCHAR(28)     NOT NULL    CHECK (
        status IN ('pending', 'processing', 'ready', 'failed')
    ),

    PRIMARY KEY (id)
);
