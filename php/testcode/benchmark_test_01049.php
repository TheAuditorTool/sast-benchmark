<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01049(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('search');

    $html = '<input type="text" value="' . $input . '">'
          . '<p>Results for: ' . $input . '</p>';

    return BenchmarkResponse::html($html);
}
