<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00796(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('content');

    $html = '<div>' . $input . '</div>';

    return BenchmarkResponse::html($html);
}
