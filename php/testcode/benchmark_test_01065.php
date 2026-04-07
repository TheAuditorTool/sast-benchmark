<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01065(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    assert(is_numeric($id));
    return BenchmarkResponse::ok("ID is valid: " . $id);
}
