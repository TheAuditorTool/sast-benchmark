<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00479(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('bio');

    $safe = strip_tags($input);
    $html = '<p>' . $safe . '</p>';

    return BenchmarkResponse::html($html);
}
