<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_unserialize_cookie
function deserial_unserialize_cookie(BenchmarkRequest $req): BenchmarkResponse {
    $sessionData = $req->cookie('session');
    $obj = unserialize($sessionData); // vuln-code-snippet vuln-line php_deser_unserialize_cookie
    return BenchmarkResponse::json(['user' => $obj->username ?? 'unknown']);
}
// vuln-code-snippet end php_deser_unserialize_cookie
