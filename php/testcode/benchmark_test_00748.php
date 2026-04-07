<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00748(BenchmarkRequest $req): BenchmarkResponse {
    $latte = new Latte\Engine();
    $latte->render('/views/template.latte', ['user' => $req->param('name')]);
    return BenchmarkResponse::ok('rendered');
}
