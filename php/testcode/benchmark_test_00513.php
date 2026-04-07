<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00513(BenchmarkRequest $req): BenchmarkResponse {
    $templates = new League\Plates\Engine(__DIR__ . '/templates');
    $output = $templates->render('profile', ['name' => $req->param('name')]);
    return BenchmarkResponse::html($output);
}
