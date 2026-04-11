# X12 Phase Roadmap v0

## Зачем этот документ

Этот документ фиксирует
не итоговую судьбу `X12`,
а **рабочий порядок фаз**,
чтобы не пытаться строить
всё сразу.

Логика простая:

- `X12` уже стал реальной системой
- теперь нужно двигаться фазами
- каждая фаза должна закрывать
  один класс задач
- не надо смешивать output,
  client content, input
  и ecosystem compatibility
  в один шаг

## Phase 1 — Server Spine and Software Manifest

Это то, что уже есть.

Сюда входит:

- `chaos -> table -> crystall -> manifest`
- `X11` wire subset `v0`
- session model
- multi-client identity
- software manifest
- damage
- PPM visual dump

Это уже:

**proof of concept that X12 is alive**

## Phase 2 — Real Output Backend

Цель этой фазы:

**показать текущий software manifest
на реальном экране**

Без:

- real client pixel content
- `PutImage`
- `SHM`
- input

Минимальный смысл фазы:

- `DRM/KMS`
- dumb buffer
- page flip
- current front buffer goes to monitor

Success:

- `X12` показывает свой manifest
  на реальном Linux display output

## Phase 3 — Client Content Path

Цель этой фазы:

**дать форме не только geometry,
но и реальное pixel content**

Сюда входит:

- backing store у формы
- copy/blit path из form content
  в manifest
- `PutImage`-style content updates
- later maybe `SHM`

Важно:

эта фаза идет **после**
real output backend,
чтобы не смешивать:

- “умеем ли мы вообще показывать кадр”
и
- “умеем ли мы принимать client pixels”

Success:

- форма содержит реальные пиксели,
  а не только id-colored rectangle

## Phase 4 — Input and Event Routing

Цель:

**сделать систему интерактивной**

Сюда входит:

- `libinput`
- focus
- keyboard/mouse delivery
- basic event routing

Важно:

input — это отдельный класс сложности.
Его не надо смешивать
с первичным output path.

Success:

- можно взаимодействовать
  с формами через input events

## Phase 5 — Deeper X11 Compatibility

Цель:

**приближать `X12` к реальному X11 world**

Сюда входит:

- расширение wire subset
- replies/events
- более глубокая X11 semantics
- bridge growth
- путь к реальным старым клиентам

Важно:

это нельзя делать вслепую
раньше, чем:

- output path
- client content path
- input path

иначе compatibility станет
больше обещанием, чем системой

Success:

- старые X11 клиенты
  начинают жить не в smoke-mode,
  а по-настоящему

## Почему Phase 2 разбита

Изначально хочется слепить:

- `DRM`
- `SHM`
- `xterm`

в одну фазу.

Это плохой порядок.

Потому что тогда непонятно,
что именно сломалось:

- output backend
- client content transport
- X11 semantics
- event flow

Поэтому:

- сначала **real output**
- потом **real client pixels**

Это дешевле для отладки
и честнее для архитектуры.

## Коротко

Рабочий порядок фаз такой:

1. software/server proof
2. real output
3. real client content
4. input
5. deeper compatibility

Это и есть текущий pragmatic roadmap
для `X12`.
