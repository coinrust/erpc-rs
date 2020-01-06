#include "common.h"
#include "util/numautils.h"

void req_handler(erpc::ReqHandle *req_handle, void *context) {
    printf("req_handler start\n");
    AppContext *c  = static_cast<AppContext *>(context);
    auto req_msgbuf = req_handle->get_req_msgbuf();
    std::string s((const char*)req_msgbuf->buf, req_msgbuf->get_data_size());
    printf("%s\n", s.c_str());
    erpc::MsgBuffer *resp = &req_handle->pre_resp_msgbuf;
    auto kMsgSize = strlen("world");
    c->rpc->resize_msg_buffer(resp, kMsgSize);
    sprintf(reinterpret_cast<char *>(resp->buf), "world");

    c->rpc->enqueue_response(req_handle, resp);
    printf("req_handler end\n");
}

void basic_sm_handler(int session_num, erpc::SmEventType sm_event_type,
                      erpc::SmErrType sm_err_type, void *_context) {
    printf("basic_sm_handler\n");
}

void server_func(erpc::Nexus *nexus, size_t thread_id) {
    AppContext c;
    erpc::Rpc<erpc::CTransport> *rpc = new erpc::Rpc<erpc::CTransport>(nexus, static_cast<void *>(&c), thread_id, basic_sm_handler);
    c.rpc = rpc;
    while (true) {
        rpc->run_event_loop(1000);
    }
}

extern "C" {

int server_test() {
    int num_threads = 2;
    std::string server_uri = kServerHostname + ":" + std::to_string(kServerUDPPort);
    printf("server_uri: %s\n", server_uri.c_str());
    erpc::Nexus nexus(server_uri, 0, 0);
    nexus.register_req_func(kReqType, req_handler);

    std::vector<std::thread> threads(num_threads);

    for (size_t i = 0; i < num_threads; i++) {
        threads[i] = std::thread(server_func, &nexus, i);
        erpc::bind_to_core(threads[i], 0, i);
    }

    for (size_t i = 0; i < num_threads; i++) threads[i].join();
}

}