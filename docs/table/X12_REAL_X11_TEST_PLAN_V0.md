# X12 Real X11 Test Plan v0

## Зачем этот документ

После:

- headless spine
- wire handshake
- request parsing
- wire -> spine integration

следующий правильный шаг —
не расширять протокол вслепую,
а прогнать `X12` через реальный `X11` client.

## Главный тезис

Нам сейчас не нужен "полный desktop test".

Нужен:

> минимальный реальный `X11` boundary test

То есть клиент,
который действительно говорит по `X11` protocol,
а не нашими внутренними тестовыми байтами.

## Что именно тестируем

### Цель

Проверить:

- что setup handshake реально понимается клиентом
- что synthetic root достаточно валиден для первого контакта
- что клиент может отправить хотя бы один поддерживаемый request
- что `X12` не врет про свой wire subset

## Какой клиент нужен

### Первый кандидат

Очень маленькая программа на:

- `Xlib`
или
- `XCB`

Лучше всего:

- свой минимальный test client
- без внешней магии
- без toolkit
- без ожидания большого event-world

## Какой сценарий брать первым

### Самый узкий сценарий

1. открыть connection
2. пройти setup
3. взять root window из setup
4. послать `CreateWindow`
5. послать `MapWindow`

Опционально:

6. послать `ConfigureWindow`

На этом first real test уже достаточно.

## Что считать успехом

Успехом `v0` считается:

- клиент не отваливается на setup сразу
- клиент реально открывает connection
- клиент реально отправляет request
- `X12` принимает этот request на boundary
- request доходит до внутреннего spine

То есть нам сейчас не нужен "window appeared on real desktop".

Нужен:

- real protocol client
- real setup
- real request path

## Где ожидаемо упрется

С высокой вероятностью реальный client упрется в одном из мест:

### 1. Setup surface

Synthetic setup может оказаться слишком бедным:

- vendor/pixmap format/root fields
- root visual assumptions
- missing setup details

### 2. Missing events

Даже если request проходит,
клиент может ждать:

- `Expose`
- `MapNotify`
- `ConfigureNotify`

а мы их пока не шлем.

### 3. Unsupported request zoo

Даже очень маленький `Xlib/XCB` путь
может внезапно послать что-то кроме наших 4 request'ов.

Это тоже нормальный результат теста.

Не провал,
а pressure-signal,
что boundary нужно расширять.

## Какой client лучше первым

### Не `xclock`

Пока не надо.

Почему:

- `xclock` уже живет как normal `X11` app
- он может ждать event flow и drawing semantics
- это уже слишком широкий pressure-test

### Лучше tiny custom client

Свой маленький test client лучше,
потому что он:

- говорит ровно то, что нам нужно
- не втаскивает hidden assumptions
- позволяет двигаться узко и честно

## Результаты теста надо делить на 3 класса

### 1. Setup failure

Клиент даже не проходит handshake.

Значит чинить:

- setup bytes
- synthetic root surface
- connection semantics

### 2. Request failure

Handshake проходит,
но request ломается.

Значит чинить:

- parser
- subset support
- X11 boundary semantics

### 3. Post-request behavioral failure

Request проходит,
но клиент ждет events/replies,
которых пока нет.

Значит следующий шаг уже:

- session semantics
- event subset

## Что делать после first real client test

Если real test client проходит:

- можно идти в multi-request session
- можно добавлять sequence correctness
- можно думать про first events

Если real test client ломается:

- фиксируем точку давления
- расширяем wire subset только настолько,
  насколько реально требуется

## Короткая формула

Следующий practical pressure-test для `X12`:

- не `xclock`
- не desktop
- не toolkit

А:

- tiny real `X11` client
- real setup
- real `CreateWindow` / `MapWindow`
- real boundary proof
