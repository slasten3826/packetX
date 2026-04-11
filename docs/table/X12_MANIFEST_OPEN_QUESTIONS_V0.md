# X12 Manifest Open Questions v0

## Зачем этот документ

Этот документ не фиксирует готовую truth.

Он фиксирует именно те вопросы,
которые сейчас полезно думать
вместе с GLM,
не лезя пока в код и не притворяясь,
что `DRM/KMS` уже понятны до деталей.

## Что уже можно считать найденным

На этом этапе уже зафиксировано:

- `manifest` лучше мыслить как поток проявления
- `packet` в `manifest` лучше мыслить как атом этого потока
- `packet = pixel * Hz` не отменяется
- `table` участвует в packet-filtering через stacking/order truth
- `crystall` дорешивает final visibility
- если `X12` на Linux сам напрямую выводит на монитор,
  то нижняя physical boundary `manifest`
  проходит через `DRM/KMS`

Ниже уже идут не найденные ответы,
а открытые узлы.

## Open Question 1

### Что является минимальной единицей manifest-flow

Пока есть несколько возможных чтений:

- пиксель
- scanline fragment
- tile
- form fragment
- packet-field fragment

Текущая интуиция склоняется к packet-атомарности,
но еще не зафиксировано,
насколько literal должен быть
`pixel * Hz` в самом `manifest`.

## Open Question 2

### Один поток или несколько

Пока не решено,
как именно мыслить packet-flow в `manifest`:

- один глобальный направленный поток
- несколько синхронных потоков
- tile-based flow
- scanline-based flow
- region-based flow

То есть потоковость уже найдена,
но topology этого потока пока не найдена.

## Open Question 3

### Где exactly режется packet-pressure

Сейчас рабочее чтение такое:

- `table` задает stacking/order truth
- `crystall` дорешивает geometry/visibility
- `manifest` уже проявляет выжившее

Но еще надо точнее понять:

- какая часть packet-kill происходит уже в `table`
- какая часть packet-kill должна жить только в `crystall`
- имеет ли `manifest` право на last-mile rejection

## Open Question 4

### Имеет ли `manifest` право дорешивать

Открытый вопрос:

- `manifest` должен быть почти тупым output layer
- или он имеет право делать last-mile decisions

Но уже можно зафиксировать одну жесткую границу:

- даже если `manifest` informed клиентом,
  output commit для `X12` остается server-controlled

Например:

- снизить throughput
- упростить output path
- пропустить часть packet-flow
- выбрать более грубый способ явления

Это напрямую связано с будущей темой FPS,
pressure и output degradation.

## Open Question 5

### Что считается успешным `manifest`

Это не до конца ясно.

Возможные чтения:

- новый кадр проявлен
- часть поля обновлена
- scanout буфер сменился
- continuity сохранена даже без видимой разницы
- "рисовать нечего" тоже valid manifest result

То есть надо понять,
что именно считать успешным актом manifest.

## Open Question 6

### Что `manifest` должен хотеть от Linux lower boundary

Пока мы не пишем backend,
но уже можно думать абстрактно:

`manifest` должен уметь потребовать у нижнего мира:

- output surface
- mode continuity
- present / flip / update
- buffer visibility
- timing discipline

То есть вопрос пока не "как писать DRM/KMS код",
а:

> какой minimal physical contract
> нужен `manifest` от Linux lower boundary

И здесь важно не спутать:

- клиент может информировать `manifest`
  о content / dirty state
- но lower-boundary contract с Linux hardware
  остается server-side responsibility

## Open Question 7

### Является ли `manifest` ближе к монитору,
### а `chaos` ближе к GPU, буквально или только онтологически

Сейчас сильная интуиция такая:

- `chaos` ближе к hardware/GPU reality
- `manifest` ближе к monitor-visible reality

Но еще надо понять,
насколько это:

- просто полезная метафора
- или уже почти literal архитектурная ось

## Open Question 8

### Нужен ли отдельный document про DRM/KMS,
### но только как manifest-boundary, а не как implementation guide

Возможно, следующим полезным шагом будет
не код и не deep Linux study,
а отдельный короткий документ:

- что для `X12` значит `DRM/KMS`
- что это не "весь мир графики",
  а только lower boundary manifest
- чего мы пока о нем не знаем

## Open Question 9

### Как соотносится `crystall policy` и server truth

Если visual/WM-like policy живет рядом с `crystall`,
то надо отдельно понять:

- это часть server core
- или privileged client logic

Сильный текущий кандидат:

- `crystall core` = server-controlled
- `crystall policy` = privileged client,
  который посылает intention в server path

## Коротко

Сейчас главное не "начать кодить вывод",
а удержать честные вопросы:

- что такое manifest-flow
- что в нем атомарно
- где packet режется
- что считается успешным явлением
- какой minimal contract нужен от Linux lower boundary
