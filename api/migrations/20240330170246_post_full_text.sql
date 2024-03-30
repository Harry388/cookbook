alter table post add fulltext(title, content) with parser ngram;
