use potassium_shot_plugin::{ApiRegister, AxumResult, BuiltApiRegister, UserId};

use crate::data::PortfolioData;

mod data;

#[unsafe(no_mangle)]
pub fn register_api(register: ApiRegister) -> BuiltApiRegister {
    register
        .name("portfolio")
        .register("/get-data", get_portfolio_data)
        .build()
}

#[unsafe(no_mangle)]
pub fn init() {}

#[unsafe(no_mangle)]
pub fn user_deleted(_id: UserId) {}

async fn get_portfolio_data(_: ()) -> AxumResult<PortfolioData> {
    PortfolioData::load()
}
