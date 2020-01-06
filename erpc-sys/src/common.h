#include <stdio.h>
#include "rpc.h"

static const std::string kServerHostname = "127.0.0.1";
static const std::string kClientHostname = "127.0.0.1";

static constexpr uint16_t kServerUDPPort = 31850;
static constexpr uint16_t kClientUDPPort = 31851;
static constexpr uint8_t kReqType = 2;
static constexpr size_t kMsgSize = 16;

class AppContext {
public:
    erpc::Rpc<erpc::CTransport> *rpc = nullptr;
    size_t thread_id;
    int session_num;
    erpc::MsgBuffer req;
    erpc::MsgBuffer resp;

    ~AppContext() {}
};
