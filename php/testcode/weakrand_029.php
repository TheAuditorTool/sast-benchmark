<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_date_small_rand
function weakrand029(BenchmarkRequest $req): BenchmarkResponse {
    $id = date('YmdHis') . rand(100, 999); // vuln-code-snippet vuln-line php_weakrand_date_small_rand
    return BenchmarkResponse::ok($id);
}
// vuln-code-snippet end php_weakrand_date_small_rand
