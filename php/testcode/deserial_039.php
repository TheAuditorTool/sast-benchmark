<?php
require_once __DIR__ . '/shared.php';

define('CACHED_CONFIG', 'a:2:{s:4:"host";s:9:"localhost";s:4:"port";i:3306;}');

// vuln-code-snippet start php_deser_hardcoded_constant
function deserial039(BenchmarkRequest $req): BenchmarkResponse {
    $config = unserialize(CACHED_CONFIG); // vuln-code-snippet safe-line php_deser_hardcoded_constant
    return BenchmarkResponse::json($config);
}
// vuln-code-snippet end php_deser_hardcoded_constant
