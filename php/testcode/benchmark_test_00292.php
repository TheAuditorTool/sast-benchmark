<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00292(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('prefs');
    parse_str($input, $out);
    $safe = $out['lang'] ?? 'en';
    return BenchmarkResponse::ok("lang=$safe");
}
