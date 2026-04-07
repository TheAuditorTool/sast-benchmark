<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00129(BenchmarkRequest $req): BenchmarkResponse {
    $tplName  = $req->param('tpl');
    $cacheFile = CACHE_DIR . sha1($tplName) . '.php';
    include $cacheFile;
    return BenchmarkResponse::ok('Cached template rendered');
}
