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
