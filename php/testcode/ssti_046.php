<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_precompiled_deploy_cache
function ssti046(BenchmarkRequest $req): BenchmarkResponse {
    $cacheFile = constant('CACHE_DIR') . '/' . sha1('user_card') . '.php';
    include $cacheFile; // vuln-code-snippet safe-line php_ssti_precompiled_deploy_cache
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_ssti_precompiled_deploy_cache
