CREATE EXTENSION IF NOT EXISTS hstore
=
CREATE EXTENSION IF NOT EXISTS hstore

CREATE EXTENSION "Foobar" VERSION '1' CASCADE
=
CREATE EXTENSION "Foobar"
  WITH VERSION '1'
       CASCADE

CREATE EXTENSION foo SCHEMA addons
=
CREATE EXTENSION foo
  WITH SCHEMA addons
