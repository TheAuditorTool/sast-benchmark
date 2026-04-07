<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01084(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('url');

    $html = '<a href="' . $input . '">Click here</a>';

    return BenchmarkResponse::html($html);
}
