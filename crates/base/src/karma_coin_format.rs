use crate::genesis_config_service::ONE_KC_IN_KCENTS;

/// Returns formatted string for displayu purposes
pub fn format_kc_amount(amount: u64) -> String {
    if amount >= (ONE_KC_IN_KCENTS / 10) {
        // 0.1 or more Karma Coins
        let kc_amount = amount as f64 / ONE_KC_IN_KCENTS as f64;
        if kc_amount > 1.0 {
            format!("{:.2} Karma Coins", kc_amount)
        } else {
            format!("{:.2} Karma Coin", kc_amount)
        }
    } else if amount == 1 {
        "One Karma Cent".into()
    } else {
        format!("{} Karma Cents", amount)
    }
}
