<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00241(BenchmarkRequest $req): BenchmarkResponse {
    $cacheFile = constant('CACHE_DIR') . '/' . sha1('user_card') . '.php';
    include $cacheFile;
    return BenchmarkResponse::ok('rendered');
}
