# test_noir_file Library

# test_noir_file Module

This module contains the following components:

## Table of Contents
- [Structs](#structs)
- [Implementations](#implementations)

## Structs

### AccountActions

Fields:
- `context`: Context
- `is_valid_impl`: fn (& mut PrivateContext , Field) -> bool

## Implementations

### Impl for AccountActions < Context >

#### `init`



```rust
fn init(context: Context, is_valid_impl: fn (& mut PrivateContext , Field) -> bool) -> Self
```

### Impl for AccountActions < & mut PrivateContext >

#### `entrypoint`

* Verifies that the `app_hash` and `fee_hash` are authorized and then executes them.
     * 
     * Executes the `fee_payload` and `app_payload` in sequence.
     * Will execute the `fee_payload` as part of the setup, and then enter the app phase.
     *

| Parameter | Type | Description |
|-----------|------|-------------|
| `app_payload` | `AppPayload` | The payload that contains the calls to be executed in the app phase. |
| `fee_payload` | `FeePayload` | The payload that contains the calls to be executed in the setup phase. |

```rust
fn entrypoint(app_payload: AppPayload, fee_payload: FeePayload)
```

#### `verify_private_authwit`

* Verifies that the `msg_sender` is authorized to consume `inner_hash` by the account.
     * 
     * Computes the `message_hash` using the `msg_sender`, `chain_id`, `version` and `inner_hash`.
     * Then executes the `is_valid_impl` function to verify that the message is authorized.
     * 
     * Will revert if the message is not authorized. 
     *

| Parameter | Type | Description |
|-----------|------|-------------|
| `inner_hash` | `Field` | The hash of the message that the `msg_sender` is trying to consume. |

```rust
fn verify_private_authwit(inner_hash: Field) -> Field
```

