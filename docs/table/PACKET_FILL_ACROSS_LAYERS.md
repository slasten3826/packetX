# Packet Fill Across Layers

## Зачем этот документ

Раньше можно было ошибочно понять `X12 Packet` как:

- просто один большой struct
- или как набор нескольких packet-типов

Но более точная модель выглядит иначе.

`X12 Packet`:

- один
- целостный
- но не статически полный с самого начала

Он **заполняется** по мере прохождения через уровни абстракции.

## Базовый тезис

`X12 Packet` начинается почти пустым в `chaos`
и становится все более определенным,
пока проходит через:

- `table`
- `crystall`
- `manifest`

То есть packet — это не просто контейнер,
а:

> растущее тело графического процесса

## Почему это важно

Это позволяет:

- не делать несколько packet'ов там, где нужен один
- не превращать packet в сразу заполненный монолит
- сохранить packet-онтологию как процесс, а не как snapshot мира

## Стадии наполнения

### `chaos`

`Chaos` не дает packet готового смысла.

Он дает:

- сырой импульс
- driver/input/output reality
- raw event
- raw pressure
- raw timing

Packet здесь:

- минимальный
- почти пустой
- неоформленный

### `table`

`Table` дописывает:

- relation truth
- capability truth
- field access truth
- legal routing possibilities
- topology facts

Packet здесь становится:

- адресуемым
- легальным
- вписанным в систему отношений

### `crystall`

`Crystall` дописывает:

- form truth
- focus truth
- visibility truth
- composition truth
- manifestation priority

Packet здесь становится:

- устойчивым
- графически определенным
- готовым к final realization

### `manifest`

`Manifest` дописывает:

- final present path
- carrier target
- scanout decision
- presentation rate
- actual realization

Packet здесь становится:

- завершенным
- явленным
- исчерпанным локально

## Что это означает для структуры packet

Секции packet не должны пониматься как:

- четыре отдельных packet'а

Они должны пониматься как:

- четыре зоны,
  которые постепенно наполняются truth'ами

То есть:

- `relation zone`
- `form zone`
- `visibility zone`
- `manifest zone`

не обязаны быть полными в начале.

## Важное различие

Packet в `X12` не просто переносит готовую истину.

Он:

- собирает истину
- удерживает ее
- и приносит ее к manifestation

## Самая короткая формула

`X12 Packet` — это один packet,
который начинается как потенциал
и наполняется truth'ами,
пока проходит через уровни системы.
