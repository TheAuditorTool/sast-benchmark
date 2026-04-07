<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00426(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    $engine = new Mustache_Engine();
    $output = $engine->render($template, ['name' => 'World']);
    return BenchmarkResponse::html($output);
}
