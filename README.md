# solana_attestation_sdk

This is an onchain utility module for verifying Switchboard Functions.

Usage:

```
#[derive(Accounts)]
pub struct Example<'info> {
    /// CHECK: todo
    #[account(constraint = validate_fn_quote(&function, &quote, &signer))]
    pub function: AccountInfo<'info>,
    /// CHECK: todo
    pub quote: AccountInfo<'info>,
    pub signer: Signer<'info>,
}
```
