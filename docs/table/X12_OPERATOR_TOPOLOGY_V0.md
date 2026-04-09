# Топология операторов X12 v0

## Status

`Mixed document / partially legacy`

Этот документ остается важным:

- topology операторов остается валидной
- mapping по слоям остается валидным

Но sections,
где packet описан как self-routing сущность,
теперь надо читать как legacy.

После атомарного поворота:

- packet тупой
- слои и операторы умные
- packet не выбирает путь сам
- packet проводится через topology

Источники:

- `stack-core/ProcessLang/microPL.txt`
- `stack-core/FOUR_LEVELS_OF_ABSTRACTION.md`

## Базовое ядро

Операторы:

- `▽ FLOW`
- `☰ CONNECT`
- `☷ DISSOLVE`
- `☵ ENCODE`
- `☳ CHOOSE`
- `☴ OBSERVE`
- `☶ LOGIC`
- `☲ CYCLE`
- `☱ RUNTIME`
- `△ MANIFEST`

Core:

- `FLOW`: `x -> f -> x'`
- `CONNECT`: `a + b -> rel`
- `DISSOLVE`: `rel -> parts`
- `ENCODE`: `x* -> pattern`
- `CHOOSE`: `{paths} -> 1`
- `OBSERVE`: `observe(x)`
- `LOGIC`: `rules(x)`
- `CYCLE`: `iterate f^n(x)`
- `RUNTIME`: `ctx -> state'`
- `MANIFEST`: `output`

## Топология соседства

Это не жесткая линейка, а graph ближайших переходов:

- `FLOW -> CONNECT -> DISSOLVE -> OBSERVE`
- `CONNECT -> FLOW -> DISSOLVE -> OBSERVE -> ENCODE`
- `DISSOLVE -> FLOW -> CONNECT -> OBSERVE -> CHOOSE`
- `OBSERVE -> FLOW -> CONNECT -> DISSOLVE -> ENCODE -> CHOOSE -> RUNTIME`
- `ENCODE -> OBSERVE -> RUNTIME -> CHOOSE -> CYCLE`
- `CHOOSE -> OBSERVE -> RUNTIME -> ENCODE -> LOGIC`
- `RUNTIME -> OBSERVE -> MANIFEST -> ENCODE -> CHOOSE -> LOGIC -> CYCLE`
- `CYCLE -> ENCODE -> LOGIC -> MANIFEST -> RUNTIME`
- `LOGIC -> CHOOSE -> CYCLE -> RUNTIME -> MANIFEST`
- `MANIFEST -> RUNTIME -> CYCLE -> LOGIC`

## Что это значит для X12

Главное: у нас теперь есть не просто список операторов, а topology law. Значит packet не должен блуждать как угодно. Он должен двигаться по допустимым соседствам.

Отсюда для `X12`:

- `FLOW` получает сырой импульс из `chaos`
- `CONNECT` вписывает relation truth
- `DISSOLVE` распускает устаревшие связи и stale state
- `OBSERVE` меряет состояние packet и решает, готов ли он к следующему шагу
- `ENCODE` кристаллизует form/scene state
- `CHOOSE` коллапсирует конфликтующие пути
- `LOGIC` проверяет допустимость коллапса и ограничений
- `CYCLE` меняет скаляры режима, но не саму сущность формы
- `RUNTIME` удерживает инерцию и state continuity
- `MANIFEST` дает финальное явление

## Топология packetX

Если переложить это на наш текущий packet, получается такая рабочая карта:

- `FLOW`
  наполняет packet сырьем из драйверов, input, hotplug, damage, timing
- `CONNECT`
  наполняет `state.relations`, capability bindings, field access
- `DISSOLVE`
  чистит stale focus, stale placement, stale visibility claims
- `OBSERVE`
  читает `metrics`, `log_ledger`, conflict flags, visibility pressure
- `ENCODE`
  наполняет `state.forms`, `state.visibility_graph`, composition-ready form
- `CHOOSE`
  выбирает route, target form, placement path, manifest priority path
- `LOGIC`
  проверяет fullscreen exclusivity, focus legality, output legality
- `CYCLE`
  крутит rate, dampening, animation intensity, low-priority manifest cadence
- `RUNTIME`
  удерживает continuity packet между тиками и сохраняет state tendency
- `MANIFEST`
  наполняет `state.scanout_contract` и доводит packet до явления

## Практический вывод

`next_module` нельзя писать произвольно.

И его не должен навязывать текущий модуль.

`packet` сам решает, куда идти дальше,
но только внутри двух ограничений:

- topology операторов `microPL`
- topology уровней `chaos -> table -> crystall -> manifest`

То есть:

- выбор делает сам `packet`
- границы выбора задают topology laws
- `fallback` тоже допустим только если он разрешен topology

Правильная формула для `packetX` теперь такая:

`next_module = packet.choose_next(pl_topology, layer_topology)`

## Топология по уровням абстракции

`FOUR_LEVELS_OF_ABSTRACTION.md` дает уже готовый частичный mapping:

### `chaos`

- `FLOW`
- `DISSOLVE`
- `CONNECT`

Это нижняя зона сырого импульса и распада. Здесь packet еще почти пустой, но уже получает:

- raw input
- driver reality
- hotplug
- timing pressure
- распад устаревших связей из прошлого тика

### `table`

- `CONNECT`
- `DISSOLVE`
- `OBSERVE`
- `CHOOSE`
- `ENCODE`

Это зона первичной раскладки и адресуемости. Здесь packet получает:

- relation truth
- capability truth
- route candidates
- field bindings
- первые кристаллизации допустимой структуры

### `crystall`

- `CHOOSE`
- `ENCODE`
- `LOGIC`
- `CYCLE`
- `RUNTIME`

Это зона устойчивой формы. Здесь packet получает:

- form truth
- visibility truth
- composition-ready state
- manifestation priority
- удержание continuity между тиками

### `manifest`

- `MANIFEST`

Это уже не сборка формы, а ее выпадение в мир:

- present path
- carrier choice
- scanout contract
- final realization

## Самая важная склейка

`microPL` дает topology graph операторов.

`FOUR_LEVELS_OF_ABSTRACTION` дает layer ownership для этих операторов.

Вместе это означает:

- packet двигается не просто между функциями
- packet двигается по graph операторов
- а этот graph уже распределен по слоям `chaos -> table -> crystall -> manifest`

То есть в `packetX`:

- topology operators
- и topology layers

это одна и та же машина, увиденная с двух сторон.

## Самый важный вывод

Топология уже есть в `microPL`.

Значит в `X12`:

- не надо выдумывать pipeline с нуля
- надо проявить packet-pipeline как manifestation этой topology
- routing packet и вообще любой `X12 Packet` должен ходить по графу операторов, а не по ad-hoc цепочке модулей
- packet не должен ходить туда, куда ему нельзя ни по topology `ProcessLang`, ни по topology четырех уровней абстракции
