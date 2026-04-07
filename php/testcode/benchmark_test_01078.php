<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01078(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    if (!ctype_digit($id)) {
        return BenchmarkResponse::badRequest('ID must be a positive integer');
    }
    return BenchmarkResponse::json(['user_id' => (int)$id]);
}
