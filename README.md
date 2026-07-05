# Description

A simple in-memory database written in Rust with a custom SQL-like grammar powered by the `pest` parser

## Struktura projektu

Projekt składa się z trzech modułów:

- parsing  
- errors  
- model

### Moduł `parsing`

Moduł odpowiedzialny jest za odczytywanie kwerendy podanej na wejściu. Na podstawie zdefiniowanej gramatyki generowany jest enum `Statement`, który reprezentuje każdą wspieraną komendę wraz z jej argumentami.

W module znajduje się główne wejście wykonywania zapytania, czyli funkcja `run_query`. Na podstawie danych wejściowych buduje ona strukturę `Command` a nastepnie wywoluje na niej funkcje `execute`.

### Moduł `model`

W folderze `model/Command` znajdują się definicje wszystkich komend. Każda komenda zawiera:

- pole `statement`, które przechowuje rezultat parsowania (enum `Statement`),
- referencję do bazy danych lub tabeli, zależnie od typu komendy,
- metodę `execute()`, która wykonuje właściwą logikę danej operacji.

### Moduł `errors`

Moduł zawiera wszystkie typy błędów wykorzystywane w projekcie oraz aliasy wyników:

- `Result<T, ParsingError>`
- `Result<T, ExecutionError>`
- `Result<T, DatabaseError>`

Główną strukturą opisującą błąd jest `DatabaseErr`, która określa:

- rodzaj błędu (parsowanie lub wykonanie),
- komendę (`Statement`), której błąd dotyczy.

---

## Komendy

**CREATE** – tworzy tabelę z nazwą, kluczem głównym oraz schematem pól.  
Przykład składni:
[CREATE|C] table [KEY|K] primary_key [FIELDS|F] key: Type, field1: Type, field2: Type

**INSERT** – dodaje rekord z wartościami zgodnymi ze schematem tabeli.  
Przykład składni:
[INSERT|I] field1 = val1, field2 = val2 [INTO|I] table

**DELETE** – usuwa rekord na podstawie wartości klucza głównego.  
Przykład składni:
[DELETE|D] key_value [FROM] table

**READ_FROM** – wczytuje i wykonuje komendy z pliku tekstowego.  
Przykład składni:
[READ_FROM|READ|R|R_F] path/to/file

Obsługiwane typy pól: `String`, `Int`, `Float`, `Bool`.  
Klucz główny musi mieć typ zgodny z wybranym rodzajem bazy (`String` lub `Int`).

Wszystkie przecinki, spacje oraz wielkość liter w komendach są opcjonalne.  
Wyjątek stanowią nazwy pól – muszą być identyczne jak w schemacie tabeli.

## Uruchamianie

## Running the Program

```bash
cargo run -- --db int
# or
cargo run -- --db string
```

---

## Ulubiony modul

Parsing z pest byl bardzo ciekawy w uzyciu