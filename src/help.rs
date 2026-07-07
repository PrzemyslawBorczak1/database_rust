/// Returns the full help text describing every supported command.
///
/// Stored as a static literal (`&'static str`) and built once into a `String`
/// so that main can simply print it.
pub fn help() -> String {
    String::from(HELP)
}

const HELP: &str = "\
=============================================================
  Simple SQL database - available commands
=============================================================

Keywords are case-insensitive.
Most of them also have a short form (shown in parentheses).

-------------------------------------------------------------
CREATE (C)  -  create a table
-------------------------------------------------------------
  CREATE <table> KEY <key> FIELDS <field>: <Type>, ...

  The primary key must be one of the fields. Types: see below.
  Example:
    CREATE scientists KEY name FIELDS name: String, publication_count: Int

-------------------------------------------------------------
INSERT (I)  -  insert a record
-------------------------------------------------------------
  INSERT <field> = <value> ... INTO (I) <table>

  Provide all fields including the primary key value.
  Example:
    INSERT name = Curie publication_count = 7 INTO scientists

-------------------------------------------------------------
SELECT  -  read records
-------------------------------------------------------------
  SELECT <columns | *> FROM (F) <table> [ORDER_BY <field>] [LIMIT <n>]

  *              all columns
  ORDER_BY <f>   sorts the result ascending by field <f>
  LIMIT <n>      limits the result to the first n records
  Both modifiers are optional and may appear in any order.

  Examples:
    SELECT painter FROM paintings LIMIT 5
    SELECT better, amount_won FROM bets ORDER_BY amount_won
    SELECT name, publication_count FROM scientists ORDER_BY publication_count LIMIT 10

-------------------------------------------------------------
DELETE (D)  -  delete a record by key
-------------------------------------------------------------
  DELETE <key> FROM (F) <table>

  Example:
    DELETE Curie FROM scientists

-------------------------------------------------------------
SAVE_AS (SAVE, S_A)  -  save the database to a file
-------------------------------------------------------------
  SAVE_AS <path>

  Example:
    SAVE_AS ./data/db.txt

-------------------------------------------------------------
READ_FROM (READ, R_F, R)  -  load the database from a file
-------------------------------------------------------------
  READ_FROM <path>

  Example:
    READ_FROM ./data/db.txt

=============================================================
  Field types
=============================================================
  String (Str)      text
  Int (Integer)     integer number
  Float             floating-point number
  Bool (Boolean)    true / false

=============================================================
  Shell commands (REPL)
=============================================================
  help              show this help
  exit              quit the program

Multiple commands can be separated by a comma on a single line.
";
