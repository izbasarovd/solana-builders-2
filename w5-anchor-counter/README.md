# Counter на Anchor (неделя 5)

Программа-счётчик на Anchor 1.x: аккаунт `Counter { authority: Pubkey, count: u64 }`, инструкции `initialize`, `increment`, `decrement`, `reset`. Менять счётчик может только `authority`.

> **Статус: задеплоено в devnet, клиент отработал на devnet.**

- Сеть: **devnet**
- Program ID: `673AWrposEH54uTFKn9fEi7L2z2jGEKG72F2CuDqG85E` — [Explorer](https://explorer.solana.com/address/673AWrposEH54uTFKn9fEi7L2z2jGEKG72F2CuDqG85E?cluster=devnet)
- Signature деплоя: `4eWzVvFzfLXLwbwX1g3QTVEngauGAxQHRFt12fZ7Za9tZNA6kBrC7v3VdwQgFmvYNpdwY8DnGyzoRWCcL1zjejWC` — [Explorer](https://explorer.solana.com/tx/4eWzVvFzfLXLwbwX1g3QTVEngauGAxQHRFt12fZ7Za9tZNA6kBrC7v3VdwQgFmvYNpdwY8DnGyzoRWCcL1zjejWC?cluster=devnet)
- Upgrade authority: `Fo9Qwfn1DybRm6sPKY5SdwSPKSqA5xDKwFuU7CQpB3fS`
- Размер `.so`: 135 896 байт
- Адрес Counter: `EYMFS9hGUKE7CEpQ5VXSQtFhWBjriBofeqTbgBasuJ8d` — [Explorer](https://explorer.solana.com/address/EYMFS9hGUKE7CEpQ5VXSQtFhWBjriBofeqTbgBasuJ8d?cluster=devnet), итоговый `count = 2`
- Потрачено SOL: деплой 0.692784 (ProgramData 0.691231 + program account 0.000833 + комиссии за загрузку ≈0.00072), клиент 0.000924; всего 0.693708. Остаток на кошельке 4.110552 SOL.
- IDL в сеть не загружался (`--no-idl`, экономия ≈0.008 SOL): клиент берёт его из `client/idl/counter.json`.

## Структура

```
programs/counter/src/
  lib.rs                   declare_id!, #[program] с 4 инструкциями
  state.rs                 #[account] #[derive(InitSpace)] struct Counter
  error.rs                 #[error_code] CounterError (CounterUnderflow = 6000, CounterOverflow = 6001)
  instructions/initialize.rs  Initialize (init, payer, space)
  instructions/update.rs      Update (mut, has_one = authority) + increment / decrement / reset
tests/counter.ts           TypeScript-тесты (mocha + chai), 7 тестов
client/client.ts           клиент; client/idl/counter.json — копия target/idl/counter.json
Anchor.toml  Cargo.toml  package.json  tsconfig.json
```

## Constraints

| Constraint | Что проверяет |
|---|---|
| `init, payer = authority, space = 8 + InitSpace` | создаёт аккаунт через System Program, rent-exempt, владелец — программа |
| `#[account(mut)]` | аккаунт изменяемый (writable) |
| `has_one = authority` | `counter.authority == authority.key()` |
| `Signer` | аккаунт подписал транзакцию |
| `Account<Counter>` | владелец == program id и discriminator совпадает |

## Команды

```bash
npm install
anchor build                                          # target/deploy/counter.so, target/idl, target/types
anchor keys sync                                      # declare_id! и Anchor.toml берут ID из target/deploy/counter-keypair.json
anchor test --provider.cluster localnet --validator legacy   # 7 тестов на локальном валидаторе, бесплатно
```

Деплой в devnet (один раз; ≈0.69 SOL, кошелёк берётся из `Anchor.toml`):

```bash
solana rent 135941                                    # стоимость ProgramData (размер .so + 45 байт заголовка)
anchor deploy --provider.cluster devnet --no-idl -- --max-len 135896
solana program show 673AWrposEH54uTFKn9fEi7L2z2jGEKG72F2CuDqG85E
CLUSTER=devnet npm run client
```

## Воспроизводимость сборки

Сборка идёт стандартным `anchor build` без нестандартных флагов компилятора (`-Z...`, `RUSTC_BOOTSTRAP` не используются). Тулчейн закреплён в `rust-toolchain.toml`. Новый Program ID появляется при первой сборке в `target/deploy/counter-keypair.json`; ключ в git не попадает (`*-keypair.json` в `.gitignore`), резервная копия лежит вне проекта в `~/solana-builders-2/_backup/`.

## Размер .so

`opt-level = "s"`, `lto = "fat"`, `codegen-units = 1`, `strip`, `panic = "abort"`, `overflow-checks = true` (`Cargo.toml`) и feature `no-log-ix-name`. Итог: 135 896 байт. Размер почти весь определяется самим Anchor (обработка ошибок, форматирование). Rent за ProgramData пропорционален размеру: 0.6912 SOL против 0.0939 SOL у нативной программы недели 4 (18 312 байт).

## Результат запуска клиента на devnet

`CLUSTER=devnet npm run client`, вывод и полные signatures — в `run-devnet.log`. Итоговый `count = 2`, authority `Fo9Qwfn1DybRm6sPKY5SdwSPKSqA5xDKwFuU7CQpB3fS`.

| Действие | Signature | count |
|---|---|---|
| initialize | [qmAuSuTj…LmmVCS](https://explorer.solana.com/tx/qmAuSuTjCg8B9p2sU8soAVk7TVVrm19qJDHVP8QMpdpZQ8ChYDmuijJd8j68X5gzz15hsvKoN6k3uYK4FLmmVCS?cluster=devnet) | count = 0 |
| increment | [gvyRWuVy…HokEuR](https://explorer.solana.com/tx/gvyRWuVyFSXPDqZFhZZuQQ7TsDdRQCs99gQMRnnvfCEpVQMyaWqs32Lq4fYGf7VmF4QZMTgnmXNAzttsGHokEuR?cluster=devnet) | count = 1 |
| increment | [27NmamFh…T2C1Q3](https://explorer.solana.com/tx/27NmamFh7vrgkimLkMctru7DELRbc9VfUkSMJq6KkW4LGnK769qxNtBKnCdiHKJAcx5PNav2M7LALZweMpT2C1Q3?cluster=devnet) | count = 2 |
| increment | [5iJQct8L…CsE2H1](https://explorer.solana.com/tx/5iJQct8LfiVpSDPoYMNFsVhBjmRJmDnv2awKfTm9Dfod26NF7XccZE7BqmzeY325HCNTKZ8NspzAaQmmppCsE2H1?cluster=devnet) | count = 3 |
| decrement | [2jS5np1Y…oZrrFL](https://explorer.solana.com/tx/2jS5np1YH9NS83azmnFhHxH7E53crJP3psvnpboJFVJRQpy6B2dyiMqByd5BwZC6wPnJwY7YDJfLQdFKfToZrrFL?cluster=devnet) | count = 2 |
