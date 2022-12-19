-- Add up migration script here
CREATE TABLE blog.article_tag
(
    article_id INT references blog.articles (id),
    tag_id     INT references blog.tags (id),
    PRIMARY KEY (article_id, tag_id)
);


INSERT INTO blog.article_tag (article_id, tag_id)
VALUES (1000, 1);
INSERT INTO blog.article_tag (article_id, tag_id)
VALUES (1000, 2);
INSERT INTO blog.article_tag (article_id, tag_id)
VALUES (1001, 2);


CREATE VIEW blog.article2tag AS
SELECT article_id, ARRAY_AGG(tag_id) as tag_ids
FROM blog.article_tag
group by article_id;

CREATE VIEW blog.tag2article AS
SELECT tag_id, ARRAY_AGG(article_id) as article_ids
FROM blog.article_tag
group by tag_id;

-- SELECT article_id FROM blog.article_tag WHERE tag_id IN (1) group by article_id having count(article_id) >= 1;
-- SELECT * FROM (SELECT * FROM blog.articles where id = 1000)  as A Left outer join blog.article2tag as B on A.id = B.article_id;
