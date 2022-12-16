-- Add up migration script here
CREATE SCHEMA blog;
CREATE TYPE blog.article_state AS ENUM ('all', 'published', 'hidden');
