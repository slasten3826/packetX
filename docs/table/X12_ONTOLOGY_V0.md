# X12 Ontology v0

## Status

`Mixed document / partially legacy`

Документ остается полезным как общий срез.

Но sections про `packet`,
написанные до атомарного поворота,
надо читать как legacy.

Каноничный новый packet-layer сейчас лежит в:

- `X12_ATOMIC_PACKET_MODEL_V0.md`
- `X12_PACKET_BIRTH_INVARIANT_V0.md`

## Назначение

Этот документ собирает текущий каркас `X12` в одну короткую форму.

Это не финальная спецификация.
Это первый стабильный срез того,
что уже удалось вытащить из исследования.

## Базовый тезис

`X12` сейчас читается не как вариант `X11` и не как вариант `Wayland`.

`X12` начинает читаться как:

> process-graphics runtime

где графическая реальность не задаётся заранее как единый экран,
а собирается и проявляется через packet-процессы, операторы и уровни абстракции.

## Уровни

### `chaos`

Слой substrate и сырой физики:

- kernel
- DRM/KMS
- input hardware
- GPU
- buffers
- sync reality
- driver-specific ограничения

Сами драйверы живут здесь.

### `table`

Слой отношений, адресации и capability truth:

- object model
- relation model
- fields/carriers discovery
- topology facts
- bridges / translators
- packet contracts

### `crystall`

Слой устойчивой графической формы.

Здесь живут:

- `field`
- `form`
- focus logic
- placement law
- window manager logic
- compositor core
- visibility preparation

Именно здесь форма удерживается как форма,
но еще не выпадает окончательно наружу.

### `manifest`

Слой финального явления:

- present
- scanout
- final output realization
- direct path
- remote/stream export

`Manifest` не должен владеть всем миром.
Он должен быть конечным актом, а не царем системы.

## Главные сущности

### `relation`

Формальная связь между процессом,
его правами,
его fields
и допустимыми manifestation paths.

### `field`

Графический домен до финального проявления.

Не экран.
Не монитор.
Не scene graph.

### `form`

Устойчивая графическая форма внутри field.

### `carrier`

Физический или логический носитель manifestation.

Примеры:

- monitor
- virtual output
- remote output

### `focus chain`

Явная структура authority для input.

### `visibility graph`

Производная структура,
показывающая,
какие forms реально могут проявляться на каких carriers.

### `manifest path`

Режим финального проявления.

Примеры:

- composed desktop path
- fullscreen direct path
- reduced-rate path
- snapshot path

## Packet

### Legacy note

Ниже packet еще читается слишком крупно,
как carrier одного графического процесса.

После большого инсайта это больше не каноничное чтение.

Теперь правильнее различать:

- `packet` как атом
- `form` как сборку packet'ов
- более крупные process/body сущности,
  которые раньше ошибочно назывались packet

`X12 Packet` — это не сообщение и не готовая сцена.

Это:

> контейнер состояния текущего графического процесса

Примеры packet-процессов:

- input routing process
- scene update process
- visibility update process
- fullscreen/direct process
- presentation process

## Операторы

Текущий рабочий маппинг на `ProcessLang`:

- `FLOW`
  сырой графический сигнал
- `CONNECT`
  структурные связи графического мира
- `DISSOLVE`
  распад устаревших связей
- `ENCODE`
  кристаллизация form/scene state
- `CHOOSE`
  выбор активного пути
- `OBSERVE`
  измерение и планирование
- `LOGIC`
  ограничения и запреты
- `CYCLE`
  настройка скаляров поведения
- `RUNTIME`
  удержание устойчивости
- `MANIFEST`
  финальный present

## Топология операторов

### Legacy note

Ниже topology еще местами описана через слишком умный packet.

После атомарного поворота это надо перечитывать так:

- packet тупой
- topology и слои умные
- packet не self-routed,
  а проводится слоями через допустимую topology

В `X12` операторы — это не просто список функций.

Они уже имеют topology law из `microPL`,
а ownership по слоям уже дан в `FOUR_LEVELS_OF_ABSTRACTION`.

Это значит:

- packet не должен ходить по произвольной цепочке
- packet должен двигаться по graph допустимых operator-переходов
- этот graph уже распределен по слоям `chaos -> table -> crystall -> manifest`

Текущий сжатый mapping по слоям:

- `chaos`
  `FLOW / DISSOLVE / CONNECT`
- `table`
  `CONNECT / DISSOLVE / OBSERVE / CHOOSE / ENCODE`
- `crystall`
  `CHOOSE / ENCODE / LOGIC / CYCLE / RUNTIME`
- `manifest`
  `MANIFEST`

Практический вывод:

`packet` сам выбирает следующий шаг,
но не произвольно.

Он не имеет права идти туда,
куда ему нельзя:

- по topology операторов `ProcessLang`
- по topology уровней `chaos -> table -> crystall -> manifest`

То есть `next_module` определяется не волей обработчика,
а topology law, прочитанным из состояния packet:

`next_module = packet.choose_next(pl_topology, layer_topology)`

## Visibility / Manifestation

В `X12` различаются:

- жизнь процесса
- жизнь формы
- частота manifestation

То, что не видно,
не обязано рисоваться с полной частотой.

Но то, что не проявляется,
не обязано переставать существовать.

## WM и Compositor

На текущем этапе они читаются не как главные сущности системы,
а как исторические имена для функций внутри `crystall`.

Текущая рабочая гипотеза:

- `window manager` = верхний `crystall`
- `compositor core` = нижний `crystall`
- финальный present = `manifest`

## Log и PU

### `log_ledger`

Оставляем как append-only лог packet-переходов:

- `DISSOLVE`
- `CHOOSE`
- `ENCODE`
- `MANIFEST`
- bridge/translation loss

### `PU`

Пока не трактуется как валюта.

Пока это:

- показатель скорости
- показатель ёмкости
- показатель того,
  насколько тяжело текущий графический процесс удерживается на данном железе

## Самая короткая формула

`X12` — это графический runtime,
в котором display reality не рисуется как данность,
а кристаллизуется как процесс и затем проявляется наружу.
