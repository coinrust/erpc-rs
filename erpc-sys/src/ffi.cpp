#include "util/latency.h"
#include <signal.h>
#include <cstring>
#include "rpc.h"
#include "util/numautils.h"
#include "transport_impl/eth_common.h"
#include "rpc_types.h"

class AppContext {
public:
    erpc::Rpc<erpc::CTransport> *rpc = nullptr;
    erpc::MsgBuffer req_msgbuf;
    erpc::MsgBuffer resp_msgbuf;

    ~AppContext() {}
};

extern "C" {

erpc::Nexus *erpc_nexus_new(const char *local_uri, size_t numa_node, size_t num_bg_threads) {
    return new erpc::Nexus(local_uri, numa_node, num_bg_threads);
}

void erpc_nexus_destroy(erpc::Nexus *nexus) {
    if (nexus != nullptr) {
        delete nexus;
        nexus = nullptr;
    }
}

void erpc_nexus_register_req_func(erpc::Nexus *nexus, uint8_t req_type, erpc::erpc_req_func_t req_func,
                                  uint8_t req_func_type) {  // ReqFuncType
    nexus->register_req_func(req_type, req_func, erpc::ReqFuncType(req_func_type));
}

AppContext *app_context_new() {
    return new AppContext{};
}

void app_context_destroy(AppContext *context) {
    if (context != nullptr) {
        delete context;
        context = nullptr;
    }
}

erpc::Rpc<erpc::CTransport> *app_context_rpc(AppContext *context) {
    return context->rpc;
}

erpc::Rpc<erpc::CTransport> *erpc_rpc_new(erpc::Nexus *nexus, AppContext *context, uint8_t rpc_id,
        erpc::sm_handler_t sm_handler,
        uint8_t phy_port) {
    auto ret = new erpc::Rpc<erpc::CTransport>(nexus, static_cast<void *>(context), rpc_id, sm_handler, phy_port);
    ret->retry_connect_on_invalid_rpc_id = true;
    context->rpc = ret;
    return ret;
}

void erpc_rpc_destroy(erpc::Rpc<erpc::CTransport> *rpc) {
    if (rpc != nullptr) {
        delete rpc;
        rpc = nullptr;
    }
}

int erpc_connect_session(erpc::Rpc<erpc::CTransport> *rpc, const char* server_uri, uint8_t rem_rpc_id) {
    int session_num = rpc->create_session(server_uri, rem_rpc_id);
    erpc::rt_assert(session_num >= 0, "Failed to create session");
    //printf("session_num %d\n", session_num);
    return session_num;
}

bool erpc_rpc_is_connected(erpc::Rpc<erpc::CTransport> *rpc, int session_num) {
    return rpc->is_connected(session_num);
}

void erpc_run_event_loop_once(erpc::Rpc<erpc::CTransport> *rpc) {
    rpc->run_event_loop_once();  // size_t timeout_ms
}

void erpc_rpc_run_event_loop(erpc::Rpc<erpc::CTransport> *rpc, size_t timeout_ms) {
    rpc->run_event_loop(timeout_ms);  // size_t timeout_ms
}

uint8_t *erpc_get_req_msgbuf(erpc::ReqHandle *req_handle, size_t &data_size) {
    auto msgbuf = req_handle->get_req_msgbuf();
    data_size = msgbuf->get_data_size();
    return msgbuf->buf;
}

void erpc_enqueue_request(AppContext *context, erpc::Rpc<erpc::CTransport> *rpc, int session_num, uint8_t req_type,
        const uint8_t *data, size_t data_size, erpc::erpc_cont_func_t cont_func, size_t tag, size_t cont_etid) {
    context->req_msgbuf = rpc->alloc_msg_buffer_or_die(data_size);
    context->resp_msgbuf = rpc->alloc_msg_buffer_or_die(rpc->get_max_msg_size());

    memcpy(context->req_msgbuf.buf, data, data_size);

    rpc->enqueue_request(session_num, 1, &context->req_msgbuf,
                           &context->resp_msgbuf, cont_func, reinterpret_cast<void *>(tag)); // nullptr
}

void erpc_enqueue_response(erpc::Rpc<erpc::CTransport> *rpc, erpc::ReqHandle *req_handle, const uint8_t *data,
        size_t data_size) {
    auto &resp = req_handle->pre_resp_msgbuf;
    rpc->resize_msg_buffer(&resp, data_size);
    memcpy(resp.buf, data, data_size);

    rpc->enqueue_response(req_handle, &resp);
}

uint8_t *erpc_get_resp_msgbuf(AppContext *context, size_t &data_size) {
    data_size = context->resp_msgbuf.get_data_size();
    return context->resp_msgbuf.buf;
}

}
