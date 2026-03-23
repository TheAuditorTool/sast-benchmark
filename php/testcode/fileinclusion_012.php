<?php
require_once __DIR__ . '/shared.php';

define('CONFIG_DIR', __DIR__ . '/config');

// vuln-code-snippet start php_fi_local_constant
function fileinclusion012(BenchmarkRequest $req): BenchmarkResponse {
    include(CONFIG_DIR . '/database.php'); // vuln-code-snippet safe-line php_fi_local_constant
    return BenchmarkResponse::ok("config loaded");
}
// vuln-code-snippet end php_fi_local_constant
