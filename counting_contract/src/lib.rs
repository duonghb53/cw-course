mod contract;
pub mod msg;

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    todo!()
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match _msg {
        Value { value } => to_binary(&query::value(value + 1)),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        execute, instantiate,
        msg::{QueryMsg, ValueResponse},
        query,
    };
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[test]
    fn query_value() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &Empty {},
                &[],
                "Counting Contract",
                None,
            )
            .unwrap();

        let resp: ValueResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value { value: 1 })
            .unwrap();

        assert_eq!(resp, ValueResponse { value: 2 });
    }
}
