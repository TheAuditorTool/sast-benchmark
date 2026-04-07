<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00757(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');

    $html = '<html><body><h1>Hello, ' . $name . '!</h1></body></html>';

    return BenchmarkResponse::html($html);
}
