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
#[inline]
pub fn allows(role: StoreMemberRole, permission: Permission) -> bool {
    match role {
        // Owner has every permission in the Permission vocabulary.
        StoreMemberRole::Owner => true,

        // Admin has operational management authority over all defined resources.
        // Ownership-sensitive actions (like ownership transfer or deletion) are not
        // part of the Permission enum.
        StoreMemberRole::Admin => true,

        // Developer has technical access (invoices, payouts, webhooks, store.read).
        // Does not have member management or store configuration updates.
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
            | Permission::WebhookDelete => true,

            Permission::StoreUpdate
            | Permission::StoreMemberRead
            | Permission::StoreMemberInvite
            | Permission::StoreMemberUpdate
            | Permission::StoreMemberRemove
            | Permission::StoreMemberSuspend
            | Permission::StoreMemberActivate => false,
        },

        // Accountant has financial read access (store.read, invoice.read, payout.read).
        // Does not have mutation permissions or webhook access.
        StoreMemberRole::Accountant => match permission {
            Permission::StoreRead | Permission::InvoiceRead | Permission::PayoutRead => true,

            Permission::StoreUpdate
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
            | Permission::WebhookDelete => false,
        },

        // Viewer has operational read-only visibility (store.read, invoice.read, payout.read, webhook.read).
        // Does not have mutation permissions or member visibility.
        StoreMemberRole::Viewer => match permission {
            Permission::StoreRead
            | Permission::InvoiceRead
            | Permission::PayoutRead
            | Permission::WebhookRead => true,

            Permission::StoreUpdate
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
            | Permission::WebhookDelete => false,
        },
    }
}
