use jamsrpay_macros::{FromDto, IntoDto, IntoProto, TryIntoDomain, dto_enum_map, proto_enum_map};

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
