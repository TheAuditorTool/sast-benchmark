<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_protobuf_typed
function deserial037(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $msg = new \Google\Protobuf\Internal\Message();
    $msg->mergeFromString($body); // vuln-code-snippet safe-line php_deser_protobuf_typed
    return BenchmarkResponse::ok('parsed');
}
// vuln-code-snippet end php_deser_protobuf_typed
