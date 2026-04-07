<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00066(BenchmarkRequest $req): BenchmarkResponse {
    $encoded = $req->param('obj');
    $decoded = base64_decode($encoded);
    $obj = unserialize($decoded);
    return BenchmarkResponse::json(['result' => $obj]);
}
