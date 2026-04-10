# X12 Server Discipline Update 0001

Дата: 2026-04-10

## Что изменилось

После появления `ClientSession`
в `X12` был сделан следующий серверный дожим.

Речь уже не про новые возможности,
а про **server discipline**.

То есть про то,
как сервер должен вести себя,
когда клиент ошибается или пытается выйти за свои границы.

## Что именно было усилено

### 1. `xError` больше не убивает всю session

Раньше:

- bad request
- server отправляет `xError`
- session завершается

Теперь:

- bad request
- server отправляет `xError`
- session продолжает жить

Это ближе к реальному `X11`-поведению.

Для `X12` это важно,
потому что server identity не должна рассыпаться
от одного кривого request.

### 2. Duplicate XID теперь режется

Раньше клиент мог:

- создать window с одним id
- затем повторно создать window с тем же id

И сервер не ловил это как отдельную ошибку.

Теперь:

- duplicate XID внутри одной session
  считается ошибкой
- server возвращает `BadValue`

Это важный кусок server truth:

- ресурс либо уже существует
- либо нет

Клиент не может дважды "родить" один и тот же resource id.

### 3. Sequence wrap больше не прыгает в `0`

Раньше `wrapping_add(1)` означал:

- `65535 -> 0`

Теперь:

- `65535 -> 1`

Это ближе к ожидаемой server semantics,
где `0` не должен внезапно становиться нормальным рабочим sequence.

## Почему это важно

Это может выглядеть как мелкий polish,
но это не polish.

Это:

- защита server identity
- защита resource truth
- защита session continuity

То есть то,
что потом иначе превращается
в очень неприятные structural bugs.

## Что подтверждено тестами

Добавлены и проходят тесты на:

- session continues after protocol error
- duplicate XID rejection
- XID range validation
- unowned resource reference rejection
- multi-request session progression

То есть это уже не "обещанное поведение",
а проверяемая server discipline.

## Новый вывод

`X12` все еще очень узкий `v0`,
но внутри своего текущего поднабора
он уже ведет себя не как one-shot wire demo,
а как более настоящий server boundary.

## Короткая формула

После этого апдейта:

- `X12` не падает от первого `xError`
- `X12` держит resource truth жестче
- `X12` ведет session ближе к настоящему серверу
