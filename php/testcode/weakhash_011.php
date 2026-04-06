<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_md4
function weakhash011(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $digest = hash('md4', $data); // vuln-code-snippet vuln-line php_weakhash_md4
    return BenchmarkResponse::json(['hash' => $digest]);
}
// vuln-code-snippet end php_weakhash_md4
