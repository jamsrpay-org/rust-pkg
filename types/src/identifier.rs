/// Generates a strongly-typed UUID identifier newtype.
///
/// Produces a `pub struct $name(Uuid)` with:
/// - `new(Uuid) -> Self`
/// - `generate() -> Self`  — creates a new v7 UUID
/// - `value(&self) -> &Uuid`
/// - `into_inner(self) -> Uuid`
/// - `parse(s: &str) -> Result<Self, uuid::Error>` — fallible parse from string
/// - `Display` / `Debug` — delegates to inner UUID
/// - `From<Uuid>` / `Into<Uuid>`
/// - `AsRef<Uuid>`
/// - `TryFrom<&str>` / `TryFrom<String>` — via `Uuid::parse_str`
/// - `Copy`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`
/// - `serde::Serialize` / `serde::Deserialize` — delegates to inner UUID
#[macro_export]
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(uuid::Uuid);

        impl $name {
            /// Wrap an existing UUID.
            #[inline]
            pub fn new(id: uuid::Uuid) -> Self {
                Self(id)
            }

            /// Generate a new unique ID (UUID v7).
            #[inline]
            pub fn generate() -> Self {
                Self(uuid::Uuid::now_v7())
            }

            /// Borrow the inner UUID.
            #[inline]
            pub fn value(&self) -> &uuid::Uuid {
                &self.0
            }

            /// Consume and return the inner UUID.
            #[inline]
            pub fn into_inner(self) -> uuid::Uuid {
                self.0
            }

            /// Parse from a UUID string, returning a `Result`.
            #[inline]
            pub fn parse(s: &str) -> Result<Self, uuid::Error> {
                uuid::Uuid::parse_str(s).map(Self)
            }
        }

        // ── Display / Debug ──────────────────────────────────────────────────

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({:?})", stringify!($name), self.0)
            }
        }

        // ── From / Into / AsRef ──────────────────────────────────────────────

        impl From<uuid::Uuid> for $name {
            #[inline]
            fn from(id: uuid::Uuid) -> Self {
                Self(id)
            }
        }

        impl From<$name> for uuid::Uuid {
            #[inline]
            fn from(id: $name) -> uuid::Uuid {
                id.0
            }
        }

        impl AsRef<uuid::Uuid> for $name {
            #[inline]
            fn as_ref(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        // ── TryFrom<&str> / TryFrom<String> ─────────────────────────────────

        impl TryFrom<&str> for $name {
            type Error = uuid::Error;

            #[inline]
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                Self::parse(s)
            }
        }

        impl TryFrom<String> for $name {
            type Error = uuid::Error;

            #[inline]
            fn try_from(s: String) -> Result<Self, Self::Error> {
                Self::parse(&s)
            }
        }

        // ── serde ────────────────────────────────────────────────────────────

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                self.0.serialize(s)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                uuid::Uuid::deserialize(d).map(Self)
            }
        }
    };
}
