<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00451(BenchmarkRequest $req): BenchmarkResponse {
    $templates = new League\Plates\Engine(__DIR__ . '/views');
    $html = $templates->render('profile', ['user' => $req->param('name')]);
    return BenchmarkResponse::ok($html);
}
