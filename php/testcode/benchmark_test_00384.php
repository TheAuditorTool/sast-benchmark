<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00384(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('user');

    $body = '<div>Hello ' . $input . '</div>';

    return BenchmarkResponse::html($body);
}
