<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00892(BenchmarkRequest $req): BenchmarkResponse {
    $vars = ['user' => $req->param('user'), 'title' => $req->param('title')];
    extract($vars);
    return BenchmarkResponse::ok('rendered');
}
