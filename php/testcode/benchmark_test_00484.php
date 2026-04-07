<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00484(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('message');

    $html = '<p>' . $input . '</p>';

    return BenchmarkResponse::html($html);
}
