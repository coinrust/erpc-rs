#include "util/latency.h"
#include <signal.h>
#include <cstring>
#include "rpc.h"
#include "util/numautils.h"
#include "transport_impl/eth_common.h"
#include "rpc_types.h"
#include "common.h"

extern "C" {

void client_cont_func(void *_c, void * _tag) {
    auto tag = reinterpret_cast<size_t>(_tag);

    auto *c = static_cast<AppContext *>(_c);
    printf("client_cont_func tag: %zu\n", tag);
}

// A basic session management handler that expects successful responses
void client_sm_handler(int session_num, erpc::SmEventType sm_event_type,
                       erpc::SmErrType sm_err_type, void *_context) {
    auto *c = static_cast<AppContext *>(_context);
//    printf("client_sm_handler session_num: %d sm_event_type: %d sm_err_type: %d\n",
//            session_num, sm_event_type, sm_err_type);
}

int connect_session(AppContext &c) {
    std::string server_uri = kServerHostname + ":" + std::to_string(kServerUDPPort); // erpc::get_uri_for_process(0);
    //printf("Process %zu: Creating session to %s.\n", FLAGS_process_id,
    //       server_uri.c_str());

    int session_num = c.rpc->create_session(server_uri, 0 /* tid */);
    erpc::rt_assert(session_num >= 0, "Failed to create session");

    while (!c.rpc->is_connected(session_num)) {
        c.rpc->run_event_loop_once();
    }

    return session_num;
}

int client_test() {
    std::string client_uri = kClientHostname + ":" + std::to_string(kClientUDPPort);
    erpc::Nexus nexus(client_uri, //erpc::get_uri_for_process(FLAGS_process_id),    // FLAGS_process_id
                      0, 0);    // FLAGS_numa_node, 0
    AppContext c;
    erpc::Rpc<erpc::CTransport> rpc(&nexus, static_cast<void *>(&c), 0,
                                    client_sm_handler, 0);  // phy_port
    rpc.retry_connect_on_invalid_rpc_id = true;
    c.rpc = &rpc;
    //c.req_msgbuf = c.rpc->alloc_msg_buffer_or_die(strlen("hello"));
    //sprintf(reinterpret_cast<char *>(c.req_msgbuf.buf), "%s", "hello");
    auto session_num = connect_session(c);

    auto kMsgSize = strlen("hello");
    auto req = rpc.alloc_msg_buffer_or_die(kMsgSize);
    auto resp = rpc.alloc_msg_buffer_or_die(kMsgSize);

    sprintf(reinterpret_cast<char *>(req.buf), "%s", "hello");

    c.rpc->enqueue_request(session_num, 1, &req,
                           &resp, client_cont_func, nullptr);
    rpc.run_event_loop(1000);
    return 0;
}

}
