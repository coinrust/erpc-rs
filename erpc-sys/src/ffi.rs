use std::os::raw::{c_int, c_char, c_void};
use libc::{size_t};

pub enum Nexus {} // erpc::Nexus
pub enum ReqHandle {} // erpc::ReqHandle
pub enum AppContext {} // AppContext
pub enum Rpc {} // erpc::Rpc<erpc::CTransport>

unsafe impl Send for Rpc {}
unsafe impl Sync for Rpc {}

/// The types of responses to a session management packet
//enum class SmErrType : int

#[doc = "< The only non-error error type"]
pub const SM_ERR_TYPE_K_NO_ERROR: SmErrType = 0;
#[doc = "< The control-path connection to the server failed"]
pub const SM_ERR_TYPE_K_SRV_DISCONNECTED: SmErrType = 1;
#[doc = "< Connect req failed because server is out of ring bufs"]
pub const SM_ERR_TYPE_K_RING_EXHAUSTED: SmErrType = 2;
#[doc = "< Connect req failed because server is out of memory"]
pub const SM_ERR_TYPE_K_OUT_OF_MEMORY: SmErrType = 3;
#[doc = "< Server failed to resolve client routing info"]
pub const SM_ERR_TYPE_K_ROUTING_RESOLUTION_FAILURE: SmErrType = 4;
#[doc = "< Connect req failed because remote RPC ID was wrong"]
pub const SM_ERR_TYPE_K_INVALID_REMOTE_RPC_ID: SmErrType = 5;
#[doc = "< Connect req failed because of transport mismatch"]
pub const SM_ERR_TYPE_K_INVALID_TRANSPORT: SmErrType = 6;
pub type SmErrType = i32;

/// Events generated for application-level session management handler
//enum class SmEventType

pub const SM_EVENT_TYPE_K_CONNECTED: SmEventType = 0;
pub const SM_EVENT_TYPE_K_CONNECT_FAILED: SmEventType = 1;
pub const SM_EVENT_TYPE_K_DISCONNECTED: SmEventType = 2;
pub const SM_EVENT_TYPE_K_DISCONNECT_FAILED: SmEventType = 3;
pub type SmEventType = u32;

#[allow(dead_code)]
extern fn sample_req_handler(_req_handle: *mut ReqHandle, _context: *mut c_void) -> () {

}

#[allow(dead_code)]
extern fn sample_sm_handler(_session_num: c_int, _sm_event_type: SmEventType, _sm_err_type: SmErrType,
                            _context: *mut c_void) {

}

#[allow(dead_code)]
extern fn sample_cont_func(_context: *mut c_void, _tag: *mut c_void) {

}

#[allow(dead_code)]
extern "C" {
    pub fn erpc_nexus_new(local_uri: *const c_char, numa_node: size_t, num_bg_threads: size_t) -> *mut Nexus;
    pub fn erpc_nexus_destroy(nexus: *mut Nexus) -> ();

    // erpc::erpc_req_func_t
    pub fn erpc_nexus_register_req_func(nexus: *mut Nexus, req_type: u8,
                                        req_func: extern fn(*mut ReqHandle, *mut c_void) -> (), req_func_type: u8);

    pub fn app_context_new() -> *mut AppContext;
    pub fn app_context_destroy(ctx: *mut AppContext) -> ();
    pub fn app_context_rpc(context: *mut AppContext) -> *mut Rpc;
    pub fn app_context_get_session_num(context: *mut AppContext) -> i32;

    // typedef void (*sm_handler_t)(int, SmEventType, SmErrType, void *);
    pub fn erpc_rpc_new(nexus: *mut Nexus, context: *mut AppContext, rpc_id: u8,
                        sm_handler: extern fn(c_int, SmEventType, SmErrType, *mut c_void), phy_port: u8) -> *mut Rpc;
    pub fn erpc_rpc_destroy(rpc: *mut Rpc) -> ();
    pub fn erpc_connect_session(context: *mut AppContext, server_uri: *const c_char, rem_rpc_id: u8) -> c_int;
    pub fn erpc_rpc_is_connected(rpc: *mut Rpc, session_num: c_int) -> bool;
    pub fn erpc_run_event_loop_once(rpc: *mut Rpc) -> ();
    pub fn erpc_rpc_run_event_loop(rpc: *mut Rpc, timeout_ms: size_t) -> ();

    pub fn erpc_get_req_msgbuf(req_handle: *mut ReqHandle, data_size: &size_t) -> *mut u8;
    pub fn erpc_enqueue_request(context: *mut AppContext, rpc: *mut Rpc, session_num: c_int, req_type: u8, data: *const u8,
                                data_size: size_t, cont_func: extern fn(*mut c_void, *mut c_void),
                                tag: size_t, cont_etid: size_t) -> ();
    pub fn erpc_enqueue_response(rpc: *mut Rpc, req_handle: *mut ReqHandle, data: *const u8, data_size: size_t) -> ();
    pub fn erpc_get_resp_msgbuf(context: *mut AppContext, data_size: &size_t) -> *mut u8;

    pub fn server_test() -> c_int;
    pub fn client_test() -> c_int;
}

// typedef void (*erpc_cont_func_t)(void *context, void *tag);