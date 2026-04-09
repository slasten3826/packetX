# X12 Packet Birth Invariant v0

## Главный тезис

`X12 Packet` может рождаться только в `chaos`.

Ни один другой слой не имеет права рождать новый packet.

## Что это значит

### `chaos`

Только `chaos` принимает внешний импульс мира:

- железо
- input devices
- drivers
- network ingress
- output events
- hotplug
- runtime wakeups, если они доходят до внешней boundary

И только `chaos` имеет право превратить этот импульс
в новый `X12 Packet`.

### `table`

`table` не рождает packet.

`table`:

- принимает packet
- вписывает relation truth
- вписывает capability truth
- задает адресуемость
- ограничивает допустимые пути

### `crystall`

`crystall` не рождает packet.

`crystall`:

- принимает packet
- оформляет form
- оформляет visibility
- оформляет focus
- оформляет composition-ready state

### `manifest`

`manifest` не рождает packet.

`manifest`:

- принимает packet
- доводит его до final present
- или завершает его без visual output

## Почему это важно

Если packet можно рождать не только в `chaos`,
то `X12` начинает терять server-shape.

Тогда:

- клиент мог бы сам создавать packet
- верхние слои могли бы самопорождено плодить packet
- граница между внешним импульсом и внутренней обработкой начала бы расплываться

Это уже был бы не сервер,
а более локальный runtime или packet-GUI.

## Серверная формула

`chaos` принимает внешний мир.

`table` раскладывает и ограничивает.

`crystall` оформляет.

`manifest` проявляет.

Но рождение нового packet происходит только в `chaos`.

## Практический вывод

Клиент не создает packet напрямую.

Клиент может только:

- послать intent
- послать update
- послать request
- послать input-like сигнал

А уже `chaos` на стороне `X12` решает,
рождается ли из этого новый packet.

## Инвариант

### Разрешено

- `chaos -> new packet`

### Не разрешено

- `table -> new packet`
- `crystall -> new packet`
- `manifest -> new packet`
- `client -> direct packet birth`

## Связь с packet lifecycle

Packet:

1. рождается в `chaos`
2. растет через `table`
3. кристаллизуется в `crystall`
4. манифестируется в `manifest`

Но не перепрыгивает точку рождения.

Рождение всегда только одно:

> `packet birth = chaos only`
