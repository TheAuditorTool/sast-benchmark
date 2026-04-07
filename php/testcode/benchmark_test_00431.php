<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00431(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('title');

    $safe = esc_html($input);
    $html = '<h1>' . $safe . '</h1>';

    return BenchmarkResponse::html($html);
}
