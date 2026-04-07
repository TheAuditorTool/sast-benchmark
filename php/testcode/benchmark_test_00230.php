<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00230(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('label');

    $html = sprintf('<p>%s</p>', $input);

    return BenchmarkResponse::html($html);
}
