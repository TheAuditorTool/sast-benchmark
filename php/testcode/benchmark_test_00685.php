<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00685(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('url');

    $safe = esc_url($input);
    $html = '<a href="' . $safe . '">Click here</a>';

    return BenchmarkResponse::html($html);
}
