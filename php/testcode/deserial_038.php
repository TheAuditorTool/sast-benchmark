<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_msgpack_scalars
function deserial038(BenchmarkRequest $req): BenchmarkResponse {
    $raw = base64_decode($req->post('data'));
    $value = msgpack_unpack($raw);
    if (!is_scalar($value) && !is_array($value)) { // vuln-code-snippet safe-line php_deser_msgpack_scalars
        return BenchmarkResponse::badRequest('unexpected type');
    }
    return BenchmarkResponse::ok(json_encode($value));
}
// vuln-code-snippet end php_deser_msgpack_scalars
