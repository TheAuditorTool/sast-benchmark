<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00397(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    $engine = new LightnCandy\LightnCandy();
    $fn = $engine::compile($input);
    $renderer = $engine::prepare($fn);
    echo $renderer(['name' => 'world']);
    return BenchmarkResponse::ok('rendered');
}
