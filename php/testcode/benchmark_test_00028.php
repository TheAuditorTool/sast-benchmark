<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00028(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('config');

    $html = '<script>var x = ' . $input . ';</script>';

    return BenchmarkResponse::html($html);
}
