<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00481(BenchmarkRequest $req): BenchmarkResponse {
    $raw = explode(',', $req->param('values'));
    $numeric = array_filter($raw, 'is_numeric');
    return BenchmarkResponse::ok(implode(',', $numeric));
}
