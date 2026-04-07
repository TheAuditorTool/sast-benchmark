<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00227(BenchmarkRequest $req): BenchmarkResponse {
    $query = $req->param('query');
    parse_str($query, $params);
    $lang = $params['lang'] ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}
