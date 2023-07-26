use cosmwasm_std::{DepsMut, Response, StdResult};

use crate::state::COUNTER;

pub fn instantiate(deps: DepsMut, counter: u64) -> StdResult<Response> {
    COUNTER.save(deps.storage, &counter)?;
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

    use crate::state::COUNTER;

    pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let counter = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &counter);

        let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());

        Ok(resp)
    }
}
