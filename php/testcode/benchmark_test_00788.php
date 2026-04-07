<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00788(BenchmarkRequest $req): BenchmarkResponse {
    $userTpl = $req->param('template');
    $templates = new League\Plates\Engine('/views');
    echo $templates->render($userTpl, []);
    return BenchmarkResponse::ok('rendered');
}
