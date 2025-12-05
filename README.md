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

- **CREATE**: tworzy tabelę z nazwą, kluczem głównym oraz schematem pól.
- **INSERT**: dodaje rekord z wartościami zgodnymi ze schematem tabeli.
- **DELETE**: usuwa rekord na podstawie wartości klucza głównego.
- **READ_FROM**: wczytuje i wykonuje komendy z pliku tekstowego.

Obsługiwane typy pól: `String`, `Int`, `Float`, `Bool`.  
Klucz główny musi mieć typ zgodny z wybranym rodzajem bazy (stringowy lub całkowitoliczbowy).