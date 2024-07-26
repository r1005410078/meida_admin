use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondHand {
    // 房屋编号
    house_id: i32,
    rent: BigDecimal,
    low_rent: BigDecimal,
    second_hand_house_id: i32,
}
