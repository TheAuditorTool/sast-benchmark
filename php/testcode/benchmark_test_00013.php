<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00013(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    $latte = new Latte\Engine();
    $html = $latte->renderToString($input, []);
    return BenchmarkResponse::ok($html);
}
