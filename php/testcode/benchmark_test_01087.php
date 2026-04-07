<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01087(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('key');
    $acl = ['admin' => 'secret123', 'editor' => 'edit456'];
    $found = array_search($key, $acl);
    if ($found !== false) {
        return BenchmarkResponse::ok("role: $found");
    }
    return BenchmarkResponse::error('denied', 403);
}
