<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00444(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    $m = new Mustache_Engine();
    $html = $m->render($input, ['user' => 'world']);
    return BenchmarkResponse::ok($html);
}
