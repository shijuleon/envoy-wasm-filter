use log::info;
use proxy_wasm;
use std::time::Duration;
use std::io::Write;

#[no_mangle]
pub fn _start() {
  proxy_wasm::set_log_level(proxy_wasm::types::LogLevel::Trace);
  proxy_wasm::set_root_context(|_| -> Box<dyn proxy_wasm::traits::RootContext> { Box::new(CustomRedisProxy) });
  proxy_wasm::set_stream_context(|_, _| -> Box<dyn proxy_wasm::traits::StreamContext>
    { 
      Box::new(CustomRedisProxy)
    });
}

struct CustomRedisProxy;

impl proxy_wasm::traits::Context for CustomRedisProxy {}

impl proxy_wasm::traits::RootContext for CustomRedisProxy {
  fn on_vm_start(&mut self, _: usize) -> bool {
    info!("on_vm_start: Initialized");
    true
  }   
}

impl proxy_wasm::traits::StreamContext for CustomRedisProxy {
  fn on_new_connection(&mut self) -> proxy_wasm::types::Action {
    info!("Got a new connection!");
    proxy_wasm::types::Action::Continue
  }   

  fn on_downstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> proxy_wasm::types::Action { 
    self.set_downstream_data(0, 28, "Hello from CustomRedisProxy!".as_bytes());
    proxy_wasm::types::Action::Continue
  }

  fn on_upstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> proxy_wasm::types::Action {
    proxy_wasm::types::Action::Continue
  }

  fn on_downstream_close(&mut self, _peer_type: proxy_wasm::types::PeerType) {
    info!("Closing downstream connection"); 
  }
}
