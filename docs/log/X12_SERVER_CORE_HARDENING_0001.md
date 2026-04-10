# X12 Server Core Hardening 0001

## Что добито

На этом проходе `X12` усилен уже не по boundary,
а по самому server core.

Сделано:

- `ServerClient` registry внутри `ServerState`
- registration клиента при session start
- `setup_done` теперь живет не только в `ClientSession`,
  но и в server registry
- cleanup клиента при disconnect
- cleanup всех форм клиента при disconnect
- graceful отказ при исчерпании xid-space
  без panic в runtime path

## Почему это важно

До этого `X12` уже умел:

- различать клиентов
- давать им разные xid-space
- держать `owner_session_id` у формы

Но сервер еще не был достаточно строгим
в собственном внутреннем lifecycle.

Теперь server truth уже знает:

- какой клиент зарегистрирован
- завершил ли он setup
- какие формы надо убрать при disconnect

## Что изменилось в поведении

### Раньше

При sequential multi-client smoke:

- клиент 1 завершал session
- его форма оставалась жить в `ServerState`
- клиент 2 мог перекрыть ее уже как следующую активную форму

Это было полезно как early proof,
но не было по-настоящему server-correct.

### Теперь

При disconnect клиента:

- сервер удаляет его из registry
- сервер удаляет его формы
- stacking пересчитывается заново

То есть следующая session уже не наследует
мертвые client-owned формы.

## Живой smoke после cleanup

Проверено еще раз через:

```bash
cargo run --bin x12-server -- --x11-client-multi /home/slasten/dev/x12/.x12-multi2.sock 2
```

И два запуска:

```bash
cargo run --bin x11_probe -- /home/slasten/dev/x12/.x12-multi2.sock session-demo
```

Что получилось:

- client 1 получил `xid_base=0x00200000`
- client 2 получил `xid_base=0x00400000`
- обе session завершились без wire errors
- каждая session в своем `last_snapshot` видит только свою форму

То есть cleanup реально работает.

## Что еще остается

Пока еще нет:

- rich per-client registry
- event queue на клиента
- `MapNotify`
- `ConfigureNotify`
- `Expose`

Но server core теперь заметно жестче,
и это уже хорошая база под events.
