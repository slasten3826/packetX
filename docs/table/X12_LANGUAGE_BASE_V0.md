# X12 Language Base v0

## Зачем этот документ

После сборки онтологии `X12`
нужно зафиксировать не только сущности,
но и языковую базу будущей реализации.

Это не финальный immutable выбор,
но это текущая рабочая позиция.

## Главный тезис

`X12` не должен строиться как чисто `C`-проект по исторической инерции.

Текущая рабочая база:

- `Rust` для server/core/hot path
- `Lua` для части policy-слоя в `crystall`

## Почему не чистый C

`X12` уже выглядит не как маленькая системная утилита,
а как большая server-shaped state machine:

- packet birth
- bridge
- client/server boundary
- layered truth
- visibility filtering
- compatibility
- server authority

Это слишком сложная форма,
чтобы держать ее только на ручной дисциплине `C`
как единственной базе.

`C` может остаться на boundary,
но не должен быть главным языком ядра.

## Почему Rust

`Rust` сейчас выбран как язык:

- серверного ядра
- low-level but structured logic
- hot path частей
- долгоживущего stateful runtime

Рабочие причины:

- лучше держит сложные состояния
- лучше ограничивает ложные partially-valid states
- лучше подходит для server/core machine,
  чем исторический `C`-монолит
- нормально живет рядом с Linux/FFI/boundary world

## Где Rust должен жить

### `chaos`

`chaos` — точно `Rust`.

Здесь:

- input ingest
- wire ingress
- driver boundary
- packet birth
- server-side hot path

### `table`

`table` — тоже `Rust`.

Здесь:

- relation truth
- legality
- security
- addressability
- compatibility boundary
- packet filtering на ранней стадии

Это слишком важный слой,
чтобы делать его динамическим с самого начала.

### `crystall core`

Нижний `crystall` тоже пока `Rust`.

Здесь:

- form assembly core
- visibility graph core
- packet filtering by overlap
- composition-ready truth

### `manifest`

`manifest` тоже `Rust`.

Здесь:

- final present
- scanout
- timing
- output realization

Это снова hot path.

## Где нужен Lua

`Lua` не как база всего `X12`,
а как policy/script layer.

### `crystall policy`

Именно здесь `Lua` выглядит уместно.

Не для ядра формы,
а для правил вокруг формы:

- stacking policy
- shell behavior
- focus policy variants
- optional composition rules
- configurable crystall laws
- experimentation layer

То есть:

- `crystall core` = `Rust`
- `crystall policy` = `Lua`

## Почему Lua уместен именно здесь

Потому что `crystall` —
это самое близкое место к:

- policy
- behavior
- style of organization
- replaceable laws формы

Именно здесь scripting может быть силой,
а не дырой в серверной truth.

## Что с C

`C` не запрещен.

Но его роль сейчас вторичная:

- FFI
- узкие low-level shims
- boundary glue
- возможно отдельные hot fragments позже

То есть не:

- `C` как язык всей системы

а:

- `C` как инструмент у границы

## Текущий расклад

- `chaos` = `Rust`
- `table` = `Rust`
- `crystall core` = `Rust`
- `crystall policy` = `Lua`
- `manifest` = `Rust`
- `C` = optional boundary/FFI layer

## Короткая формула

`Rust` держит кости и мышцы `X12`.  
`Lua` дает подвижность policy в `crystall`.  
`C` остается инструментом у границы, а не языком судьбы всей системы.
