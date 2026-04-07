<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01127(BenchmarkRequest $req): BenchmarkResponse {
    extract($_REQUEST);
    $lang = $lang ?? 'en';
    $theme = $theme ?? 'default';
    return BenchmarkResponse::ok("lang=$lang theme=$theme");
}
