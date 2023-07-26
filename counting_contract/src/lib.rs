mod contract;
pub mod msg;
mod state;
use msg::{ExecMsg, InstantiateMsg};

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

use crate::contract::exec;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, msg.counter)?;

    Ok(Response::new())
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecMsg) -> StdResult<Response> {
    match msg {
        ExecMsg::Poke {} => exec::poke(deps, info),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_binary(&query::value(deps)?),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        execute, instantiate,
        msg::{ExecMsg, InstantiateMsg, QueryMsg, ValueResponse},
        query,
    };
    use cosmwasm_std::{Addr, Empty, Response};
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
                &InstantiateMsg { counter: 100 },
                &[],
                "Counting Contract",
                None,
            )
            .unwrap();

        let resp: ValueResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResponse { value: 100 });
    }

    #[test]
    fn poke() {
        let mut app = App::default();
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter: 0 },
                &[],
                "Counting Contract",
                None,
            )
            .unwrap();
        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecMsg::Poke {},
            &[],
        )
        .unwrap();

        let resp: ValueResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResponse { value: 1 });
    }
}
