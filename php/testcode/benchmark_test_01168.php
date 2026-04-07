<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01168(BenchmarkRequest $req): BenchmarkResponse {
    $query = $req->param('q');
    $display = htmlspecialchars($query, ENT_QUOTES, 'UTF-8');
    return BenchmarkResponse::html('<p>Results for: ' . $display . '</p>');
}
