<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00708(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->post('data');
    $data = unserialize($raw, ['allowed_classes' => false]);
    if (!is_array($data)) {
        return BenchmarkResponse::badRequest('expected array');
    }
    return BenchmarkResponse::json($data);
}
