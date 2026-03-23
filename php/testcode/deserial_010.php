<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_msgpack
function deserial_msgpack(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->bodyStr();
    $data = msgpack_unpack($raw); // vuln-code-snippet safe-line php_deser_msgpack
    return BenchmarkResponse::json(['items' => $data]);
}
// vuln-code-snippet end php_deser_msgpack
