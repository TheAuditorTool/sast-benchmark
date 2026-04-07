<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00574(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('config');

    $encoded = json_encode($input);
    $html = '<script>var x = ' . $encoded . ';</script>';

    return BenchmarkResponse::html($html);
}
