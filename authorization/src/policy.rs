use crate::{Permission, StoreMemberRole};

/// Evaluates whether a [`StoreMemberRole`] is permitted to perform the operation represented by [`Permission`].
///
/// This policy evaluation is pure, synchronous, deterministic, and side-effect free.
/// It contains no database, network, async, or external state dependencies.
///
/// # Policy Decision Table
///
/// | Permission             | Owner | Admin | Developer | Accountant | Viewer |
/// |:-----------------------|:-----:|:-----:|:---------:|:----------:|:------:|
/// | store.read             |   ✓   |   ✓   |     ✓     |     ✓      |   ✓    |
/// | store.update           |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.delete           |   ✓   |   ✗   |     ✗     |     ✗      |   ✗    |
/// | store.member.read      |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.member.invite    |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.member.update    |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.member.remove    |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.member.suspend   |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | store.member.activate  |   ✓   |   ✓   |     ✗     |     ✗      |   ✗    |
/// | invoice.read           |   ✓   |   ✓   |     ✓     |     ✓      |   ✓    |
/// | invoice.create         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | invoice.update         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | invoice.cancel         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | payout.read            |   ✓   |   ✓   |     ✓     |     ✓      |   ✓    |
/// | payout.create          |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | payout.cancel          |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | webhook.read           |   ✓   |   ✓   |     ✓     |     ✗      |   ✓    |
/// | webhook.create         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | webhook.update         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | webhook.delete         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | api_key.read           |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | api_key.create         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | api_key.update         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | api_key.revoke         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
/// | api_key.rotate         |   ✓   |   ✓   |     ✓     |     ✗      |   ✗    |
#[inline]
pub fn allows(role: StoreMemberRole, permission: Permission) -> bool {
    match role {
        // Owner has every permission in the Permission vocabulary.
        StoreMemberRole::Owner => true,

        // Admin has operational management authority over all resources, except
        // store deletion which is restricted to store owners.
        StoreMemberRole::Admin => match permission {
            Permission::StoreDelete => false,
            _ => true,
        },

        // Developer has technical access (invoices, payouts, webhooks, api_keys, store.read).
        // Does not have member management or store configuration updates/deletion.
        StoreMemberRole::Developer => match permission {
            Permission::StoreRead
            | Permission::InvoiceRead
            | Permission::InvoiceCreate
            | Permission::InvoiceUpdate
            | Permission::InvoiceCancel
            | Permission::PayoutRead
            | Permission::PayoutCreate
            | Permission::PayoutCancel
            | Permission::WebhookRead
            | Permission::WebhookCreate
            | Permission::WebhookUpdate
            | Permission::WebhookDelete
            | Permission::ApiKeyRead
            | Permission::ApiKeyCreate
            | Permission::ApiKeyUpdate
            | Permission::ApiKeyRevoke
            | Permission::ApiKeyRotate => true,

            Permission::StoreUpdate
            | Permission::StoreDelete
            | Permission::StoreMemberRead
            | Permission::StoreMemberInvite
            | Permission::StoreMemberUpdate
            | Permission::StoreMemberRemove
            | Permission::StoreMemberSuspend
            | Permission::StoreMemberActivate => false,
        },

        // Accountant has financial read access (store.read, invoice.read, payout.read).
        // Does not have mutation permissions, webhook access, or API key access.
        StoreMemberRole::Accountant => match permission {
            Permission::StoreRead | Permission::InvoiceRead | Permission::PayoutRead => true,

            Permission::StoreUpdate
            | Permission::StoreDelete
            | Permission::StoreMemberRead
            | Permission::StoreMemberInvite
            | Permission::StoreMemberUpdate
            | Permission::StoreMemberRemove
            | Permission::StoreMemberSuspend
            | Permission::StoreMemberActivate
            | Permission::InvoiceCreate
            | Permission::InvoiceUpdate
            | Permission::InvoiceCancel
            | Permission::PayoutCreate
            | Permission::PayoutCancel
            | Permission::WebhookRead
            | Permission::WebhookCreate
            | Permission::WebhookUpdate
            | Permission::WebhookDelete
            | Permission::ApiKeyRead
            | Permission::ApiKeyCreate
            | Permission::ApiKeyUpdate
            | Permission::ApiKeyRevoke
            | Permission::ApiKeyRotate => false,
        },

        // Viewer has operational read-only visibility (store.read, invoice.read, payout.read, webhook.read).
        // Does not have mutation permissions, member visibility, or API key access.
        StoreMemberRole::Viewer => match permission {
            Permission::StoreRead
            | Permission::InvoiceRead
            | Permission::PayoutRead
            | Permission::WebhookRead => true,

            Permission::StoreUpdate
            | Permission::StoreDelete
            | Permission::StoreMemberRead
            | Permission::StoreMemberInvite
            | Permission::StoreMemberUpdate
            | Permission::StoreMemberRemove
            | Permission::StoreMemberSuspend
            | Permission::StoreMemberActivate
            | Permission::InvoiceCreate
            | Permission::InvoiceUpdate
            | Permission::InvoiceCancel
            | Permission::PayoutCreate
            | Permission::PayoutCancel
            | Permission::WebhookCreate
            | Permission::WebhookUpdate
            | Permission::WebhookDelete
            | Permission::ApiKeyRead
            | Permission::ApiKeyCreate
            | Permission::ApiKeyUpdate
            | Permission::ApiKeyRevoke
            | Permission::ApiKeyRotate => false,
        },
    }
}
