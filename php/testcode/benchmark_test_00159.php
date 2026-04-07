<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00159(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    $latte = new Latte\Engine();
    $latte->setTempDirectory('/tmp/latte');
    $output = $latte->renderToString($template, ['name' => 'World']);
    return BenchmarkResponse::html($output);
}
