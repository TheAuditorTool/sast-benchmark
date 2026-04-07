<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00551(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = $req->param('tpl');
    include 'http://cdn.example.com/' . $tpl;
    return BenchmarkResponse::ok('Template loaded');
}
