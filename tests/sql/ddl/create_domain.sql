CREATE DOMAIN foo integer
CREATE DOMAIN "Foo" integer CONSTRAINT "Non_Negative" CHECK (value > 0)
CREATE DOMAIN foo varchar(10) not null default 'null'
CREATE DOMAIN foo varchar(10) collate "it_IT" default 'null'
