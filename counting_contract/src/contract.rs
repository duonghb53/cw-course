use cosmwasm_std::{Coin, DepsMut, Response, StdResult};

use crate::state::{COUNTER, MINIMAL_DONATION};

pub fn instantiate(deps: DepsMut, counter: u64, minimal_donation: Coin) -> StdResult<Response> {
    COUNTER.save(deps.storage, &counter)?;
    MINIMAL_DONATION.save(deps.storage, &minimal_donation)?;
    Ok(Response::new())
}

pub mod query {
    use crate::state::COUNTER;
    use cosmwasm_std::{Deps, StdResult};

    use crate::msg::ValueResponse;

    pub fn value(deps: Deps) -> StdResult<ValueResponse> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResponse { value })
    }
}

pub mod exec {
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

    use crate::state::{COUNTER, MINIMAL_DONATION};

    pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let counter = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &counter);

        let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());

        Ok(resp)
    }

    pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let mut counter = COUNTER.load(deps.storage)?;
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;

        if info.funds.iter().any(|coin| {
            coin.denom == minimal_donation.denom && coin.amount >= minimal_donation.amount
        }) {
            counter += 1;
            COUNTER.save(deps.storage, &counter)?;
        }

        let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());

        Ok(resp)
    }
}
