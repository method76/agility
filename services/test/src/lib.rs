#![deny(
    missing_debug_implementations,
    missing_docs,
    unsafe_code,
    bare_trait_objects
)]

extern crate exonum;
#[macro_use]
extern crate exonum_derive;
extern crate protobuf;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use schema::Schema;

use exonum::{
    api::ServiceApiBuilder,
    blockchain::{self, Transaction, TransactionSet},
    crypto::Hash,
    helpers::fabric::{self, Context},
    messages::RawTransaction,
    storage::Snapshot,
};

use transactions::WalletTransactions;

/// Unique service ID.
const TEST_SERVICE_ID: u16 = 127;
/// Name of the service.
const SERVICE_NAME: &str = "test";

/// Exonum `Service` implementation.
#[derive(Default, Debug)]
pub struct Service;

impl blockchain::Service for Service {
    fn service_id(&self) -> u16 {
        TEST_SERVICE_ID
    }
    fn service_name(&self) -> &str {
        SERVICE_NAME
    }
    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        api::PublicApi::wire(builder);
    }
}

/// A configuration service creator for the `NodeBuilder`.
#[derive(Debug)]
pub struct ServiceFactory;

impl fabric::ServiceFactory for ServiceFactory {
    fn service_name(&self) -> &str {
        SERVICE_NAME
    }
    fn make_service(&mut self, _: &Context) -> Box<dyn blockchain::Service> {
        Box::new(Service)
    }
}
