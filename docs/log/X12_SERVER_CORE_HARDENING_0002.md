# X12 Server Core Hardening 0002

## Что дожато

Этот проход уже не добавлял новых server features.
Он дожал внутреннюю согласованность server state.

Сделано:

- убран duplicate `setup_done` state из `ClientSession`
- `setup_done` теперь считается server-side truth
  в `ServerState.clients`
- `mark_client_setup_done` теперь строгий:
  возвращает `bool`, а не молча игнорирует missing client
- `handle_client_session` теперь держит cleanup guard
  и чистит session даже если выход произошел по error path

## Почему это важно

До этого:

- `setup_done` жил в двух местах
- cleanup был честным на normal paths,
  но часть error paths все еще полагалась
  на ручной вызов cleanup

Теперь:

- source of truth для setup state один
- cleanup привязан к самому lifecycle session
  через guard

То есть server core стал более стабильным
не по фичам, а по дисциплине состояния.

## Что это значит practically

Теперь `X12` уже лучше держит инвариант:

> если session зарегистрирована в server state,
> то у нее есть один server-side lifecycle,
> и cleanup не зависит от того,
> насколько аккуратно вышел код выше по стеку.

## Что дальше

После этого следующий честный шаг
уже не очередной server-polish,
а первые `X11` events:

- `MapNotify`
- `ConfigureNotify`
- `Expose`
