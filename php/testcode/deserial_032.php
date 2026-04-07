<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_msgpack_cookie
function deserial032(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->cookie('state');
    $state = msgpack_unpack(base64_decode($raw)); // vuln-code-snippet vuln-line php_deser_msgpack_cookie
    return BenchmarkResponse::json(['state' => $state]);
}
// vuln-code-snippet end php_deser_msgpack_cookie
