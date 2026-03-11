mod connection;
pub mod process;
pub mod session;
pub mod transport;
pub mod contexts;
pub mod network;

mod listeners;
pub mod error;
pub mod events;

pub use session::BidiSession;
pub use connection::find_free_port;
pub use network::NetworkRequest;
pub use transport::WebsocketConnectionTransport;
pub use listeners::CommandResponseState;
pub use contexts::BrowsingContext;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
