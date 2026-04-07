<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00156(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('data');
    $html = "<img src=\"data:text/html,$input\">";
    return BenchmarkResponse::html($html);
}
