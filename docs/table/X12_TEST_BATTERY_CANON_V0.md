# X12 Test Battery Canon v0

## Зачем этот документ

`X12` уже вышел из стадии,
где можно полагаться
только на intuition и разовые smoke-test'ы.

Код растет:

- вглубь
- вширь
- по слоям
- по boundary paths

Это значит,
что баги все чаще будут жить
не в главной идее,
а в:

- lifecycle paths
- edge cases
- mixed modes
- invalid inputs
- silent ignores

Поэтому для `X12`
тесты теперь считаются
не optional hygiene,
а **обязательной частью разработки**.

## Главный принцип

Каждый новый capability в `X12`
должен не только работать,
но и **входить в накапливаемую test battery**.

То есть:

- нашли баг
- починили баг
- добавили тест
- тест остается навсегда

Смысл не в том,
чтобы каждый раз заново
придумывать проверки,
а в том,
чтобы собирать
**regression battery X12**

## Что считается обязательным

Для каждого нового шага,
который меняет поведение системы,
должно быть минимум:

### 1. Functional test

Проверяет:

- базовый рабочий путь
- новая возможность реально работает

Примеры:

- окно создается
- `PPM` дамп пишется
- wire request успешно парсится

### 2. Edge / invariant test

Проверяет:

- invalid input
- ownership
- cleanup
- no-op path
- mixed flags
- killed packet
- visibility edge case

То есть:

не только “работает”,
но и “не врет по краям”.

### 3. Manual smoke,
если меняется живой boundary

Нужен,
если трогаем:

- CLI
- wire ingress
- session lifecycle
- PPM / visual output
- future DRM/KMS

Это не заменяет автотесты.
Это отдельный live proof.

## Что именно накапливаем

`X12` должен собирать
не случайные тесты,
а **постоянную test battery**:

- wire tests
- session tests
- manifest tests
- lifecycle tests
- CLI parse tests
- visual smoke procedures

Каждый новый найденный баг
должен попадать
в эту батарею.

## Что теперь считается плохой практикой

Для `X12` теперь считается плохой практикой:

- пушить новый behavior без теста
- чинить баг без regression test
- надеяться только на manual smoke
- надеяться, что reviewer поймает edge case
- оставлять silent ambiguity в CLI/wire paths без проверки

## Pre-push review gate

Перед push нужно задавать
минимум 5 вопросов:

1. Что здесь является state mutation?
2. Кто должен узнать об этой мутации?
3. Есть ли invalid path?
4. Есть ли no-op / clean path?
5. Добавлен ли regression test,
   если здесь уже был найден баг?

Если ответ “нет”
на последний вопрос,
работа считается незавершенной.

## Как это выглядит на практике

Правильный цикл:

1. изменить код
2. добавить functional test
3. добавить edge/invariant test
4. прогнать общую test battery
5. если менялся boundary —
   сделать manual smoke
6. только потом push

## Почему это важно именно для X12

`X12` —
не маленькая утилита
и не игрушечный prototype.

Это display/server system,
и она уже растет
в нескольких измерениях сразу:

- protocol
- session
- ownership
- compositing
- output

Без жесткой test battery
такой код начнет расползаться
быстрее, чем docs и архитектура
успеют это удерживать.

## Коротко

Для `X12`
тесты теперь считаются:

**обязательным каноном разработки,
а не дополнительной вежливостью.**

Новый код без теста —
не завершенный шаг.

Починенный баг без regression test —
не закрытый баг.
