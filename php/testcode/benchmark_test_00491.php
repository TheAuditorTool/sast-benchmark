<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00491(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');

    $safe = htmlspecialchars($name, ENT_QUOTES, 'UTF-8');
    $html = '<html><body><h1>Hello, ' . $safe . '!</h1></body></html>';

    return BenchmarkResponse::html($html);
}
