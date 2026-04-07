<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01030(BenchmarkRequest $req): BenchmarkResponse {
    $engine = new Mustache_Engine([
        'loader' => new Mustache_Loader_FilesystemLoader(__DIR__ . '/templates'),
    ]);
    $output = $engine->render('greeting', ['name' => $req->param('name')]);
    return BenchmarkResponse::html($output);
}
