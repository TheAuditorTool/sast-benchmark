<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_microtime_base36
function weakrand030(BenchmarkRequest $req): BenchmarkResponse {
    $id = base_convert(str_replace('.', '', microtime(true)), 10, 36); // vuln-code-snippet vuln-line php_weakrand_microtime_base36
    return BenchmarkResponse::ok($id);
}
// vuln-code-snippet end php_weakrand_microtime_base36
