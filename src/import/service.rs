// Copyright 2018 TiKV Project Authors. Licensed under Apache-2.0.

use grpcio::{RpcStatus, RpcStatusCode};

use super::Error;

pub fn make_rpc_error(err: Error) -> RpcStatus {
    RpcStatus::new(RpcStatusCode::UNKNOWN, Some(format!("{:?}", err)))
}

#[macro_export]
macro_rules! send_rpc_response {
    ($res:ident, $sink:ident, $label:ident, $timer:ident) => {async move {
        let res = match $res {
            Ok(resp) => {
                IMPORT_RPC_DURATION
                    .with_label_values(&[$label, "ok"])
                    .observe($timer.saturating_elapsed_secs());
                $sink.success(resp)
            }
            Err(e) => {
                IMPORT_RPC_DURATION
                    .with_label_values(&[$label, "error"])
                    .observe($timer.saturating_elapsed_secs());
                $sink.fail(make_rpc_error(e))
            }
        };
        if let Err(e) = res.await {
            warn!("send rpc response"; "err" => %e);
        }
    }};
}
