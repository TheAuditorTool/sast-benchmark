<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00240(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->post('content');

    $safe = wp_kses_post($input);
    $html = '<article>' . $safe . '</article>';

    return BenchmarkResponse::html($html);
}
