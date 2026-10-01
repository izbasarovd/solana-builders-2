# Counter — первая Solana Program (devnet, нативная, без Anchor)

Программа хранит в аккаунте счётчик `struct Counter { count: u64, authority: Pubkey }` (40 байт, формат borsh).
Инструкции: `Initialize` (count = 0, authority = signer) и `Increment` (count += 1, только для authority).

- Сеть: **devnet**; все транзакции подписаны кошельком `Fo9Qwfn1DybRm6sPKY5SdwSPKSqA5xDKwFuU7CQpB3fS`
- Program ID: `BHSWBpmTGDcp2BZbzar1NKtcbZxSM9k5JH25MU1twqo1` — [Explorer](https://explorer.solana.com/address/BHSWBpmTGDcp2BZbzar1NKtcbZxSM9k5JH25MU1twqo1?cluster=devnet)
- Размер .so: 18 312 байт; upgrade authority: `Fo9Qwfn1DybRm6sPKY5SdwSPKSqA5xDKwFuU7CQpB3fS`
- Signature деплоя: `3M8QNEhQePTdu4qFmjLaiH6xVJ5dfwQjdusxE3U3iHniqHrMXa23zbYHUNXqjoiuXHCsDGYomKATefebFyK1DmMk` — [Explorer](https://explorer.solana.com/tx/3M8QNEhQePTdu4qFmjLaiH6xVJ5dfwQjdusxE3U3iHniqHrMXa23zbYHUNXqjoiuXHCsDGYomKATefebFyK1DmMk?cluster=devnet)

## Структура

```
src/lib.rs          модули, no-entrypoint feature
src/entrypoint.rs   точка входа (entrypoint_no_alloc!) и panic-handler
src/instruction.rs  enum CounterInstruction, unpack/pack
src/state.rs        struct Counter, unpack/pack_into (borsh-совместимый формат)
src/error.rs        enum CounterError + impl From<CounterError> for ProgramError
src/processor.rs    Initialize / Increment и проверки
tests/counter.rs    solana-program-test (настоящий .so)
client/counter.ts   TypeScript-клиент (@solana/web3.js)
```

## Проверки в программе

владелец счётчика == program_id · счётчик writable · authority подписал · размер данных == 40 · authority совпадает с записанным · `checked_add` · повторный Initialize запрещён.

## Сборка, тесты, деплой

```bash
cargo build-sbf                                   # target/deploy/counter.so
SBF_OUT_DIR=$PWD/target/deploy cargo test         # 12 тестов: 7 unit + 5 solana-program-test
solana program deploy target/deploy/counter.so --max-len 18312
solana program show BHSWBpmTGDcp2BZbzar1NKtcbZxSM9k5JH25MU1twqo1
```

## Клиент

```bash
cd client && npm install
npx tsx counter.ts                 # создаёт счётчик, Initialize, 3× Increment (подписывает ~/solana-builders-2/w1-wallet/keys/main.json)
npx tsx counter.ts <COUNTER>       # прочитать count
```

Клиент на TypeScript: `@solana/web3.js` проще Rust-клиента (не нужен второй крейт, быстрый запуск), а формат state читается библиотекой borsh.

## Результат запуска клиента

Полный вывод: `run-devnet.log`. Счётчик: `7xEfStoadtSQTuw72SsdCQ61Dx3tPVbWt3P6JZyD5CtU` ([Explorer](https://explorer.solana.com/address/7xEfStoadtSQTuw72SsdCQ61Dx3tPVbWt3P6JZyD5CtU?cluster=devnet)). Итоговый count = **3** (перечитан отдельным запросом).

| Действие | Signature | Результат |
|---|---|---|
| CreateAccount (System Program) | [3buPMnjZ…B49hUZ](https://explorer.solana.com/tx/3buPMnjZBCkSpHPqe9yS4YuKG8AVwrPx414unpETojzbBRLwKC6cRL9W2RTTkKnDwMzdoaD1fpTCpMBBDbB49hUZ?cluster=devnet) | 40 байт, rent 853440 lamports |
| Initialize | [45aBrgQk…WTBZin](https://explorer.solana.com/tx/45aBrgQkx8YJMZzo7KAt19WZCtmvygoNBZVnkKSZDM2tVQNJHBRuJCweFhVQrdb3XxLeeKoyDGHKvVNM4aWTBZin?cluster=devnet) | count = 0 |
| Increment #1 | [25nXjXTp…H5wTR1](https://explorer.solana.com/tx/25nXjXTp3nr3zA9B29xe81DLnC7BXL3hgAe9uZ7MakidZsLvuWfCTHqfSR5kipuAZPLYtzS2W9qVqfQhxtH5wTR1?cluster=devnet) | count = 1 |
| Increment #2 | [4kP6HB9n…g1pWAj](https://explorer.solana.com/tx/4kP6HB9nhsLLe6Kr6heN6vgTbiWFZDzTN4FuCaiAByuT9agFNT8T6a6s2KcBWp1vWPkwWU9rgEAHPnKwr7g1pWAj?cluster=devnet) | count = 2 |
| Increment #3 | [2uPokbWe…QGyDwG](https://explorer.solana.com/tx/2uPokbWewQBNsYh5dW7KnpevQu7FcJi79SRyofkP4KZWz86GfKNgC6xsRBK9uD7ftdLCQh2aeDMeQtUS8TQGyDwG?cluster=devnet) | count = 3 |

Потрачено: деплой 0.09485192 SOL (из них 0.0939038 SOL — rent, возвращается при закрытии программы) + клиент 0.00088344 SOL. Остаток на кошельке: 4.80425964 SOL.
Ключ программы в git не попадает (`*-keypair.json` и `keys/` в `.gitignore`); резервная копия лежит вне проекта в `~/solana-builders-2/_backup/`.
