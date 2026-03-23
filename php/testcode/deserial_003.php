<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_unserialize_post
function deserial_unserialize_post(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $obj = unserialize($data); // vuln-code-snippet vuln-line php_deser_unserialize_post
    return BenchmarkResponse::json(['result' => $obj]);
}
// vuln-code-snippet end php_deser_unserialize_post
