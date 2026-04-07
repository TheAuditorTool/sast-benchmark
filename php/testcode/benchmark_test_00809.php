<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00809(BenchmarkRequest $req): BenchmarkResponse {
    $varName = $req->param('vars');
    $$varName = 'injected';
    $data = compact($$varName);
    return BenchmarkResponse::json($data);
}
