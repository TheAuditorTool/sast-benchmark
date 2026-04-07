<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00391(BenchmarkRequest $req): BenchmarkResponse {
    import_request_variables('gpc', '');
    $lang = $lang ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}
