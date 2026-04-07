<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00694(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('text');
    $safe = nl2br(htmlspecialchars($input, ENT_QUOTES, 'UTF-8'));
    return BenchmarkResponse::html("<p>$safe</p>");
}
