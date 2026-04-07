<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01166(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    $safe = htmlspecialchars($name, ENT_QUOTES, 'UTF-8');
    return BenchmarkResponse::html('<h1>Hello, ' . $safe . '</h1>');
}
