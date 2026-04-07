<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00294(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('text');
    header("Content-Security-Policy: default-src 'self'");
    $safe = htmlspecialchars($input, ENT_QUOTES, 'UTF-8');
    return BenchmarkResponse::html("<p>$safe</p>");
}
