alter table recipe add fulltext(title, description) with parser ngram;
