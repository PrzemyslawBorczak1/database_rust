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

Przykład:

```sql
CREATE scientists KEY name FIELDS name: String, publication_count: Int
```

**INSERT** – dodaje rekord z wartościami zgodnymi ze schematem tabeli.  
Przykład składni:
[INSERT|I] field1 = val1, field2 = val2 [INTO|I] table

Przykład:

```sql
INSERT name = Curie, publication_count = 7 INTO scientists
```

**SELECT** – odczytuje rekordy z tabeli. Można wskazać konkretne kolumny lub `*` (wszystkie).  
Przykład składni:
SELECT [columns | *] [FROM|F] table [ORDER_BY field] [LIMIT n]

- `ORDER_BY field` – sortuje wynik rosnąco po podanym polu,
- `LIMIT n` – ogranicza wynik do pierwszych `n` rekordów,
- oba modyfikatory są opcjonalne i mogą wystąpić w dowolnej kolejności.

Przykłady:

```sql
SELECT painter FROM paintings LIMIT 5
SELECT better, amount_won FROM bets ORDER_BY amount_won
SELECT name, publication_count FROM scientists ORDER_BY publication_count LIMIT 10
```

**DELETE** – usuwa rekord na podstawie wartości klucza głównego.  
Przykład składni:
[DELETE|D] key_value [FROM] table

**SAVE_AS** – zapisuje log wszystkich zapytań wykonanych w sesji do pliku (jedno zapytanie na linię, jako poprawny SQL).  
Przykład składni:
[SAVE_AS|SAVE|S_A] path/to/file

**READ_FROM** – wczytuje i wykonuje komendy z pliku tekstowego (tak, jakby zostały wpisane w powłoce).  
Przykład składni:
[READ_FROM|READ|R|R_F] path/to/file

Dzięki temu `SAVE_AS` i `READ_FROM` tworzą parę zapisz/wczytaj – plik zapisany przez `SAVE_AS` można ponownie odtworzyć przez `READ_FROM`.

Obsługiwane typy pól: `String`, `Int`, `Float`, `Bool`.  
Klucz główny musi mieć typ zgodny z wybranym rodzajem bazy (`String` lub `Int`).

Wszystkie przecinki, spacje oraz wielkość liter w komendach są opcjonalne.  
Wyjątek stanowią nazwy pól – muszą być identyczne jak w schemacie tabeli.

### Komendy powłoki 

- `help` – wyświetla listę wszystkich poleceń wraz z opisem i przykładami,
- `exit` – kończy działanie programu.

Przykładowe skrypty z zapytaniami (do użycia przez `READ_FROM`) znajdują się w katalogu
[`src/example_files`](src/example_files), m.in. `scientists_select.txt`, `bets_paintings_select.txt`, `hott_select.txt`.

## Uruchamianie

## Running the Program

```bash
cargo run -- --db int
# or
cargo run -- --db string
```

---
