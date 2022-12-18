-- Add down migration script here
DROP VIEW blog.article2tag;
DROP VIEW blog.tag2article;

DROP TABLE blog.article_tag;
