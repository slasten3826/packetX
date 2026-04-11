# X12 X11 Compatibility Boundary Rule v0

## Зачем этот документ

`X12` с самого начала растет
с обязательной совместимостью с `X11`.

Это правильно.

Но у этой задачи есть
главный архитектурный риск:

**совместимость может начать
пожирать ядро новой системы.**

Этот документ фиксирует правило,
которое должно это запретить.

## Главное правило

`X11` должен быть
**полностью поддержан на boundary**,
но `X11` **не должен становиться
онтологией ядра `X12`.**

Если коротко:

**X11 compatibility must live at the boundary,
not recolonize the core.**

## Что это означает practically

### На boundary

`X11` должен получать
привычную поверхность:

- окна
- map / unmap
- configure
- stacking effects
- manager-like behavior
- compositor-like consequences

То есть старый клиентский мир
не должен ломаться только потому,
что внутри `X12`
мир устроен иначе.

### Внутри ядра

`X12` не должен
возвращать обратно
legacy-сущности как центр мира:

- `window manager`
- `compositor`

В ядре остаются:

- `chaos`
- `table`
- `crystall`
- `manifest`

Именно они описывают мир.

## Где живет compatibility bridge

`X11 compatibility bridge`
может жить
на ingress-краю `chaos`,
потому что именно там
старый мир входит в систему.

Но это нужно понимать точно:

**bridge может быть attached to chaos ingress,
но не должен быть частью сущности chaos.**

То есть bridge должен быть:

- подключаемым
- заменяемым
- снимаемым

Если удалить `X11 bridge`,
`X12` не должен потерять:

- packet birth
- server truth
- form logic
- visibility logic
- manifestation logic

Он должен потерять только:

- legacy compatibility path

Это и есть критерий того,
что совместимость действительно
остается на boundary,
а не врастает в ядро.

## Что является допустимым компромиссом

Допустимо:

- эмулировать legacy behavior
- проецировать `X12` truth
  в `X11`-совместимую поверхность
- выражать manager/compositor behavior
  как policy role
  или privileged client behavior

Недопустимо:

- строить ядро заново
  по образу старого `X11`
- впускать старые сущности
  обратно как source of truth
- превращать compatibility layer
  в хозяина системы

## Что это значит для window manager

Внутри `X12`
`window manager`
не должен возвращаться
как обязательная центральная сущность.

Но на boundary
`X11`-мир должен иметь возможность
видеть manager-like behavior.

Это значит:

- WM-compatible behavior должен быть
  representable
- но не как core ontology
- а как role / policy / privileged actor

## Что это значит для compositor

Внутри `X12`
`compositor` тоже не должен
возвращаться как отдельный “царь картинки”.

Его legacy-функции уже
разложены по слоям:

- `table`
- `crystall`
- `manifest`

Но на boundary
старый мир должен получать
compositor-like consequences:

- stacking results
- visibility results
- final output consequences

То есть:

compositor behavior может быть совместим,
не будучи отдельной сущностью ядра.

## Долгосрочная позиция

На переходном этапе
`X12` обязан говорить со старым миром.

Но долгосрочная цель такая:

- старые термины
  `window manager`
  и `compositor`
  становятся legacy names
- новая система мыслится
  уже по слоям `X12`

То есть:

совместимость нужна
не для сохранения старой онтологии,
а для перехода
от старого мира к новому.

## Коротко

`X12` должен быть:

- fully X11-compatible at the boundary
- fully X12-shaped in the core

Если это правило нарушить,
совместимость перестанет быть мостом
и станет паразитом ядра.
