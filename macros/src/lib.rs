//! # jamsrpay-macros
//!
//! Declarative mapping macros and boundary conversion traits for JamsrPay microservices.
//!
//! Provides zero-dependency, type-safe bidirectional enum mapping between Domain models
//! and boundary DTOs (REST / Protobuf) and database persistence representations
//! while strictly adhering to Domain-Driven Design (DDD).

// ============================================================================
// Boundary Conversion Traits & Generator Macro
// ============================================================================

/// Infallible conversion from a Domain type to an external REST DTO.
pub trait IntoDto<T> {
    fn into_dto(self) -> T;
}

/// Infallible conversion from an external REST DTO to a Domain type.
pub trait FromDto<T>: Sized {
    fn from_dto(dto: T) -> Self;
}

/// Infallible conversion from a Domain type to an external Protobuf message/enum.
pub trait IntoProto<T> {
    fn into_proto(self) -> T;
}

/// Fallible conversion from an external Protobuf or DTO type to a Domain type.
pub trait TryIntoDomain<T>: Sized {
    type Error;
    fn try_into_domain(self) -> Result<T, Self::Error>;
}

/// Infallible conversion from a Domain type to a static database string representation.
pub trait IntoPersistence {
    fn into_persistence(self) -> &'static str;
}

impl<T: IntoPersistence + Copy> IntoPersistence for &T {
    fn into_persistence(self) -> &'static str {
        (*self).into_persistence()
    }
}

/// Fallible conversion from a database string representation to a Domain type.
pub trait FromPersistence: Sized {
    type Error;
    fn from_persistence(value: &str) -> Result<Self, Self::Error>;
}

/// Defines local boundary conversion traits (`IntoProto`, `TryIntoDomain`, `IntoDto`, and `FromDto`)
/// inside the calling crate's Anti-Corruption Layer, fully satisfying Rust's orphan rules when
/// mapping between external domain types (e.g. `jamsrpay-types`) and external protobuf types (e.g. `jamsrpay-proto`).
#[macro_export]
macro_rules! define_boundary_traits {
    () => {
        pub trait IntoProto<T> {
            fn into_proto(self) -> T;
        }

        pub trait TryIntoDomain<T>: Sized {
            type Error;
            fn try_into_domain(self) -> Result<T, Self::Error>;
        }

        pub trait IntoDto<T> {
            fn into_dto(self) -> T;
        }

        pub trait FromDto<T>: Sized {
            fn from_dto(dto: T) -> Self;
        }
    };
}

/// Defines local persistence conversion traits (`IntoPersistence` and `FromPersistence`)
/// inside the calling crate's Anti-Corruption Layer, fully satisfying Rust's orphan rules.
#[macro_export]
macro_rules! define_persistence_traits {
    () => {
        pub trait IntoPersistence {
            fn into_persistence(self) -> &'static str;
        }

        impl<T: IntoPersistence + Copy> IntoPersistence for &T {
            fn into_persistence(self) -> &'static str {
                (*self).into_persistence()
            }
        }

        pub trait FromPersistence: Sized {
            type Error;
            fn from_persistence(value: &str) -> Result<Self, Self::Error>;
        }
    };
}

// ============================================================================
// Declarative Enum Mapping Macros
// ============================================================================

/// Generates bidirectional conversion traits (`IntoDto`, `FromDto`, and `TryIntoDomain`)
/// between a domain enum and an external REST DTO enum.
#[macro_export]
macro_rules! dto_enum_map {
    (
        $domain:ty, $dto:ty {
            $($d_var:ident <=> $dto_var:ident),+ $(,)?
        }
    ) => {
        impl IntoDto<$dto> for $domain {
            fn into_dto(self) -> $dto {
                match self {
                    $( <$domain>::$d_var => <$dto>::$dto_var, )+
                }
            }
        }

        impl FromDto<$dto> for $domain {
            fn from_dto(dto: $dto) -> Self {
                match dto {
                    $( <$dto>::$dto_var => <$domain>::$d_var, )+
                }
            }
        }

        impl TryIntoDomain<$domain> for $dto {
            type Error = std::convert::Infallible;

            fn try_into_domain(self) -> Result<$domain, Self::Error> {
                Ok(FromDto::from_dto(self))
            }
        }
    };

    (
        $domain:ty => $dto:ty {
            $($d_var:ident <=> $dto_var:ident),+ $(,)?
        }
    ) => {
        $crate::dto_enum_map!($domain, $dto { $($d_var <=> $dto_var),+ });
    };
}

/// Generates conversion traits (`IntoProto` and fallible `TryIntoDomain`)
/// between a domain enum and an external protobuf enum.
#[macro_export]
macro_rules! proto_enum_map {
    // 1. Infallible Domain -> Protobuf only (comma)
    (
        $domain:ty, $proto:ty {
            $($d_var:ident => $p_var:ident),+ $(,)?
        }
    ) => {
        impl IntoProto<$proto> for $domain {
            fn into_proto(self) -> $proto {
                match self {
                    $( <$domain>::$d_var => <$proto>::$p_var, )+
                }
            }
        }
    };

    // 1b. Infallible Domain -> Protobuf only (=>)
    (
        $domain:ty => $proto:ty {
            $($d_var:ident => $p_var:ident),+ $(,)?
        }
    ) => {
        $crate::proto_enum_map!($domain, $proto { $($d_var => $p_var),+ });
    };

    // 2. Bidirectional Domain <-> Protobuf with closure error (comma)
    (
        $domain:ty, $proto:ty,
        error: $err_type:ty = |$err_arg:ident| $err_expr:expr,
        {
            $($d_var:ident <=> $p_var:ident),+ $(,)?
        }
    ) => {
        impl IntoProto<$proto> for $domain {
            fn into_proto(self) -> $proto {
                match self {
                    $( <$domain>::$d_var => <$proto>::$p_var, )+
                }
            }
        }

        impl TryIntoDomain<$domain> for $proto {
            type Error = $err_type;

            fn try_into_domain(self) -> Result<$domain, Self::Error> {
                match self {
                    $( <$proto>::$p_var => Ok(<$domain>::$d_var), )+
                    $err_arg => Err($err_expr),
                }
            }
        }
    };

    // 2b. Bidirectional Domain <-> Protobuf with closure error (=>)
    (
        $domain:ty => $proto:ty,
        error: $err_type:ty = |$err_arg:ident| $err_expr:expr,
        {
            $($d_var:ident <=> $p_var:ident),+ $(,)?
        }
    ) => {
        $crate::proto_enum_map!(
            $domain, $proto,
            error: $err_type = |$err_arg| $err_expr,
            { $($d_var <=> $p_var),+ }
        );
    };

    // 3. Bidirectional Domain <-> Protobuf with direct error expr (comma)
    (
        $domain:ty, $proto:ty,
        error: $err_type:ty = $err_expr:expr,
        {
            $($d_var:ident <=> $p_var:ident),+ $(,)?
        }
    ) => {
        impl IntoProto<$proto> for $domain {
            fn into_proto(self) -> $proto {
                match self {
                    $( <$domain>::$d_var => <$proto>::$p_var, )+
                }
            }
        }

        impl TryIntoDomain<$domain> for $proto {
            type Error = $err_type;

            fn try_into_domain(self) -> Result<$domain, Self::Error> {
                match self {
                    $( <$proto>::$p_var => Ok(<$domain>::$d_var), )+
                    _ => Err($err_expr),
                }
            }
        }
    };

    // 3b. Bidirectional Domain <-> Protobuf with direct error expr (=>)
    (
        $domain:ty => $proto:ty,
        error: $err_type:ty = $err_expr:expr,
        {
            $($d_var:ident <=> $p_var:ident),+ $(,)?
        }
    ) => {
        $crate::proto_enum_map!(
            $domain, $proto,
            error: $err_type = $err_expr,
            { $($d_var <=> $p_var),+ }
        );
    };

    // 4. Shorthand with closure (comma)
    (
        $domain:ty, $proto:ty, $err_type:ty, |$err_arg:ident| $err_expr:expr, {
            $($d_var:ident <=> $p_var:ident),+ $(,)?
        }
    ) => {
        $crate::proto_enum_map!(
            $domain, $proto,
            error: $err_type = |$err_arg| $err_expr,
            {
                $($d_var <=> $p_var),+
            }
        );
    };

    // 4b. Shorthand with closure (=>)
    (
        $domain:ty => $proto:ty, $err_type:ty, |$err_arg:ident| $err_expr:expr, {
            $($d_var:ident <=> $p_var:ident),+ $(,)?
        }
    ) => {
        $crate::proto_enum_map!(
            $domain, $proto,
            error: $err_type = |$err_arg| $err_expr,
            {
                $($d_var <=> $p_var),+
            }
        );
    };

    // 5. Shorthand with direct error expr (comma)
    (
        $domain:ty, $proto:ty, $err_type:ty, $err_expr:expr, {
            $($d_var:ident <=> $p_var:ident),+ $(,)?
        }
    ) => {
        $crate::proto_enum_map!(
            $domain, $proto,
            error: $err_type = $err_expr,
            {
                $($d_var <=> $p_var),+
            }
        );
    };

    // 5b. Shorthand with direct error expr (=>)
    (
        $domain:ty => $proto:ty, $err_type:ty, $err_expr:expr, {
            $($d_var:ident <=> $p_var:ident),+ $(,)?
        }
    ) => {
        $crate::proto_enum_map!(
            $domain, $proto,
            error: $err_type = $err_expr,
            {
                $($d_var <=> $p_var),+
            }
        );
    };
}

/// Generates `IntoPersistence` and `FromPersistence` implementations for an enum.
///
/// - The `into_persistence` match arm is strictly exhaustive (no wildcard `_`), ensuring
///   a compile-time error if any new variant is added to the enum without updating
///   the persistence mapper.
/// - The `from_persistence` implementation parses the value using zero-allocation
///   case-insensitive comparison (`eq_ignore_ascii_case`) on trimmed strings.
/// - Supports optional alias lists for backwards-compatible or alternative DB representations.
#[macro_export]
macro_rules! define_persistence_enum {
    // 1. Shorthand with error variant (e.g. InvalidInvoiceStatus) using InfraError in scope
    (
        $enum_ty:ty, $err_variant:ident {
            $(
                $variant:ident => $db_str:expr $(, [ $($alias:expr),* $(,)? ])?
            ),+ $(,)?
        }
    ) => {
        impl IntoPersistence for $enum_ty {
            fn into_persistence(self) -> &'static str {
                match self {
                    $( <$enum_ty>::$variant => $db_str, )+
                }
            }
        }

        impl FromPersistence for $enum_ty {
            type Error = InfraError;

            fn from_persistence(value: &str) -> Result<Self, Self::Error> {
                let trimmed = value.trim();
                $(
                    if trimmed.eq_ignore_ascii_case($db_str)
                        $( $( || trimmed.eq_ignore_ascii_case($alias) )* )?
                    {
                        return Ok(<$enum_ty>::$variant);
                    }
                )+
                Err(InfraError::$err_variant(value.to_string()))
            }
        }
    };

    // 2. Custom error type and closure: error: ErrType = |val| ErrExpr
    (
        $enum_ty:ty, error: $err_type:ty = |$err_arg:ident| $err_expr:expr, {
            $(
                $variant:ident => $db_str:expr $(, [ $($alias:expr),* $(,)? ])?
            ),+ $(,)?
        }
    ) => {
        impl IntoPersistence for $enum_ty {
            fn into_persistence(self) -> &'static str {
                match self {
                    $( <$enum_ty>::$variant => $db_str, )+
                }
            }
        }

        impl FromPersistence for $enum_ty {
            type Error = $err_type;

            fn from_persistence(value: &str) -> Result<Self, Self::Error> {
                let trimmed = value.trim();
                $(
                    if trimmed.eq_ignore_ascii_case($db_str)
                        $( $( || trimmed.eq_ignore_ascii_case($alias) )* )?
                    {
                        return Ok(<$enum_ty>::$variant);
                    }
                )+
                let $err_arg = value;
                Err($err_expr)
            }
        }
    };
}
