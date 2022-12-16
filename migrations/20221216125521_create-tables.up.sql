-- Add up migration script here
CREATE TABLE blog.categories
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE blog.tags
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE blog.articles
(
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(255)       NOT NULL,
    content     TEXT               NOT NULL,
    summary     VARCHAR(255)       NOT NULL,
    state       blog.article_state NOT NULL default 'published',
    created_at  TIMESTAMP          NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP          NOT NULL DEFAULT NOW(),
    category_id INT references blog.categories (id)
);

CREATE TABLE blog.article_tag
(
    article_id INT references blog.articles (id),
    tag_id     INT references blog.tags (id),
    PRIMARY KEY (article_id, tag_id)
);


INSERT INTO blog.categories (name)
VALUES ('undefined-category');
INSERT INTO blog.categories (name)
VALUES ('Test_category1');

INSERT INTO blog.tags (name)
VALUES ('Test_tag0');
INSERT INTO blog.tags (name)
VALUES ('Test_tag1');

INSERT INTO blog.articles (id, title, content, summary, category_id)
VALUES (1000, 'test_title', 'test_content', 'test_summary', 1);

INSERT INTO blog.article_tag (article_id, tag_id)
VALUES (1000, 1);
