#include "common.h"
#include "rpc.h"
#include "rpc_types.h"
#include "transport_impl/eth_common.h"
#include "util/latency.h"
#include "util/numautils.h"
#include <cstring>
#include <signal.h>

extern "C"
{

  void client_cont_func(void *_c, void *_tag)
  {
    printf("client_cont_func start\n");
    auto tag = reinterpret_cast<size_t>(_tag);
    auto *c = static_cast<AppContext *>(_c);
    std::string s((const char *)c->resp.buf, c->resp.get_data_size());
    printf("tag: %zu %s\n", tag, s.c_str());
    printf("client_cont_func end\n");
  }

  // A basic session management handler that expects successful responses
  void client_sm_handler(int session_num, erpc::SmEventType sm_event_type,
                         erpc::SmErrType sm_err_type, void *_context)
  {
    auto *c = static_cast<AppContext *>(_context);
    printf("client_sm_handler session_num: %d sm_event_type: %d sm_err_type: %d\n",
           session_num, (int)sm_event_type, (int)sm_err_type);
  }

  int connect_session(AppContext &c)
  {
    std::string server_uri =
        kServerHostname + ":" + std::to_string(kServerUDPPort);
    printf("connect session server_url: %s\n", server_uri.c_str());

    int session_num = c.rpc->create_session(server_uri, 0 /* tid */);
    erpc::rt_assert(session_num >= 0, "Failed to create session");

    while (!c.rpc->is_connected(session_num))
    {
      c.rpc->run_event_loop_once();
    }

    return session_num;
  }

  void send_q(AppContext *c)
  {
    auto kReqMsgSize = strlen("hello");
    c->req = c->rpc->alloc_msg_buffer_or_die(kReqMsgSize);
    c->resp = c->rpc->alloc_msg_buffer_or_die(kMsgSize);

    sprintf(reinterpret_cast<char *>(c->req.buf), "%s", "hello");

    c->rpc->enqueue_request(c->session_num, kReqType, &c->req, &c->resp,
                            client_cont_func, nullptr);
  }

  void client_func(erpc::Nexus *nexus, size_t thread_id)
  {
    AppContext c;
    erpc::Rpc<erpc::CTransport> rpc(nexus, static_cast<void *>(&c), thread_id,
                                    client_sm_handler, 0); // phy_port
    rpc.retry_connect_on_invalid_rpc_id = true;
    c.rpc = &rpc;
    c.thread_id = thread_id;

    auto session_num = connect_session(c);
    printf("session_num %d\n", session_num);

    c.session_num = session_num;

    while (true)
    {
      rpc.run_event_loop(1000);
      send_q(&c);
    }
  }

  int client_test()
  {
    int num_threads = 2;
    std::string client_uri =
        kClientHostname + ":" + std::to_string(kClientUDPPort);
    erpc::Nexus nexus(client_uri, 0, 0);

    std::vector<std::thread> threads(num_threads);

    for (size_t i = 0; i < num_threads; i++)
    {
      threads[i] = std::thread(client_func, &nexus, i);
      erpc::bind_to_core(threads[i], 0, i);
    }

    for (size_t i = 0; i < num_threads; i++)
      threads[i].join();
    return 0;
  }

  // int client_test_sample() {
  //     std::string client_uri = kClientHostname + ":" +
  //     std::to_string(kClientUDPPort); erpc::Nexus nexus(client_uri,
  //     //erpc::get_uri_for_process(FLAGS_process_id),    // FLAGS_process_id
  //                       0, 0);    // FLAGS_numa_node, 0
  //     AppContext c;
  //     erpc::Rpc<erpc::CTransport> rpc(&nexus, static_cast<void *>(&c), 0,
  //                                     client_sm_handler, 0);  // phy_port
  //     rpc.retry_connect_on_invalid_rpc_id = true;
  //     c.rpc = &rpc;
  //     auto session_num = connect_session(c);

  //     auto kMsgSize = strlen("hello");
  //     auto req = rpc.alloc_msg_buffer_or_die(kMsgSize);
  //     auto resp = rpc.alloc_msg_buffer_or_die(kMsgSize);

  //     sprintf(reinterpret_cast<char *>(req.buf), "%s", "hello");

  //     c.rpc->enqueue_request(session_num, kReqType, &req,
  //                            &resp, client_cont_func, nullptr);
  //     rpc.run_event_loop(10000);
  //     return 0;
  // }
}
