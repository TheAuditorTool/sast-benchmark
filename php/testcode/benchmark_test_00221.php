<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00221(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('avatar');

    $html = '<img src=' . $input . ' alt="avatar">';

    return BenchmarkResponse::html($html);
}
