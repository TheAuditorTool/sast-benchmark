<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00979(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('text');
    $safe = htmlentities($input, ENT_QUOTES | ENT_HTML5, 'UTF-8');
    return BenchmarkResponse::html("<span>$safe</span>");
}
