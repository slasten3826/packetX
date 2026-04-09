# Примитивы X12 v0

## Назначение

Этот документ отвечает на вопрос:

если `Wayland` держит ключевые обязанности внутри compositor-core,
то какие именно примитивы `X12` могли бы взять эти обязанности на себя,
не воссоздавая тот же скрытый центр?

## Проблемные узлы

- input routing
- focus management
- output placement
- scene graph ownership
- direct scanout

## Примитив 1: `relation`

Формальная связь между:

- клиентским процессом
- одним или несколькими graphic fields
- seat или seat-group
- набором capabilities
- разрешенными manifestation paths

Назначение:

- задать, к чему процесс подключен
- задать, где он может проявляться
- задать его права взаимодействия

## Примитив 2: `field`

Графический домен, существующий до финальной manifestation.

`Field` не является:

- монитором
- surface
- окном
- scene graph

`Field` нужен, чтобы не скатываться обратно в:

- одно глобальное desktop space
- одну compositor-owned scene как истину

## Примитив 3: `carrier`

Физический или логический носитель manifestation.

Примеры:

- физический монитор
- виртуальный output
- remote display endpoint
- target для записи/стрима

## Примитив 4: `focus chain`

Формальная структура, описывающая, как внимание и input-rights текут через:

- seat
- relation
- field
- form

## Примитив 5: `placement law`

Устойчивое правило уровня `crystall`, которое связывает:

- fields
- forms
- carriers

не предполагая сначала одно глобальное пространство.

## Примитив 6: `form`

Устойчивая видимая графическая организация внутри field.

Примеры:

- обычное окно приложения
- группа окон
- shell layer
- tiling region
- floating form

## Примитив 7: `manifest path`

Формальный режим финальной реализации.

Примеры:

- composed desktop path
- fullscreen direct path
- video path
- kiosk path
- mirror path

## Примитив 8: `crystallization event`

Переход, в котором pending client state становится устойчивой form.

## Примитив 9: `visibility graph`

Производная структура, описывающая, какие forms сейчас видимы каким carriers
через какие manifestation paths.

## Примитив 10: `scanout contract`

Boundary contract между manifestation logic и output-механикой на границе chaos.

## Карта замещения

| Схлопнутая обязанность Wayland | Замещение в X12 |
|---|---|
| input routing | `relation` + `focus chain` + `field` |
| focus management | `focus chain` + `relation` + `form` |
| output placement | `placement law` + `field` + `carrier` |
| scene graph ownership | `form` + `visibility graph` + `manifest path` |
| direct scanout | `manifest path` + `carrier` + `scanout contract` |
