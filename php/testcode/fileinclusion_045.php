<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_compiled_cache
function fileinclusion045(BenchmarkRequest $req): BenchmarkResponse {
    $tplName  = $req->param('tpl');
    $cacheFile = CACHE_DIR . sha1($tplName) . '.php'; // vuln-code-snippet safe-line php_fi_compiled_cache
    include $cacheFile;
    return BenchmarkResponse::ok('Cached template rendered');
}
// vuln-code-snippet end php_fi_compiled_cache
