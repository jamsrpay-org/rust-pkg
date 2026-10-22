use jamsrpay_macros::{
    FromDto, FromPersistence, IntoDto, IntoPersistence, IntoProto, TryIntoDomain,
    define_persistence_enum, dto_enum_map, proto_enum_map,
};

// Mock Domain Enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OrderStatus {
    Pending,
    Processing,
    Completed,
    Cancelled,
}

// Mock REST DTO Enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OrderStatusDto {
    Pending,
    Processing,
    Completed,
    Cancelled,
}

// Mock Protobuf Enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PbOrderStatus {
    Unspecified = 0,
    Pending = 1,
    Processing = 2,
    Completed = 3,
    Cancelled = 4,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MockError(&'static str);

#[derive(Debug, PartialEq, Eq)]
pub enum InfraError {
    InvalidOrderStatus(String),
}

// Test DTO mapping
dto_enum_map!(OrderStatus => OrderStatusDto {
    Pending    <=> Pending,
    Processing <=> Processing,
    Completed  <=> Completed,
    Cancelled  <=> Cancelled,
});

// Test Protobuf mapping
proto_enum_map!(OrderStatus => PbOrderStatus, MockError, MockError("Invalid status"), {
    Pending    <=> Pending,
    Processing <=> Processing,
    Completed  <=> Completed,
    Cancelled  <=> Cancelled,
});

// Test Persistence mapping (with InfraError shorthand)
define_persistence_enum!(OrderStatus, InvalidOrderStatus {
    Pending => "pending", ["pnd"],
    Processing => "processing",
    Completed => "completed", ["done"],
    Cancelled => "cancelled",
});

#[test]
fn test_dto_bidirectional_roundtrip() {
    let domain = OrderStatus::Completed;
    let dto: OrderStatusDto = domain.into_dto();
    assert_eq!(dto, OrderStatusDto::Completed);

    let roundtrip: OrderStatus = FromDto::from_dto(dto);
    assert_eq!(roundtrip, OrderStatus::Completed);

    let try_domain: OrderStatus = dto.try_into_domain().expect("infallible conversion");
    assert_eq!(try_domain, OrderStatus::Completed);
}

#[test]
fn test_proto_bidirectional_roundtrip() {
    let domain = OrderStatus::Processing;
    let proto: PbOrderStatus = domain.into_proto();
    assert_eq!(proto, PbOrderStatus::Processing);

    let roundtrip: OrderStatus = proto.try_into_domain().expect("valid domain conversion");
    assert_eq!(roundtrip, OrderStatus::Processing);

    let invalid = PbOrderStatus::Unspecified.try_into_domain();
    assert_eq!(invalid, Err(MockError("Invalid status")));
}

#[test]
fn test_persistence_enum_mapping() {
    let domain = OrderStatus::Completed;
    assert_eq!(domain.into_persistence(), "completed");

    // Exact match
    assert_eq!(
        OrderStatus::from_persistence("completed").unwrap(),
        OrderStatus::Completed
    );
    // Case-insensitive match
    assert_eq!(
        OrderStatus::from_persistence("PROCESSING").unwrap(),
        OrderStatus::Processing
    );
    // Alias match
    assert_eq!(
        OrderStatus::from_persistence("done").unwrap(),
        OrderStatus::Completed
    );
    assert_eq!(
        OrderStatus::from_persistence("PND").unwrap(),
        OrderStatus::Pending
    );
    // Failure
    assert_eq!(
        OrderStatus::from_persistence("unknown"),
        Err(InfraError::InvalidOrderStatus("unknown".to_string()))
    );
}
