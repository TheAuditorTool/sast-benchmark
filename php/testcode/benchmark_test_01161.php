<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01161(BenchmarkRequest $req): BenchmarkResponse {
    $query = $req->param('q');
    $html = '<p>Results for: ' . $query . '</p>';
    return BenchmarkResponse::html($html);
}
