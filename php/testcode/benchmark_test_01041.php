<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01041(BenchmarkRequest $req): BenchmarkResponse {
    $config_db = 'production';
    $config_debug = false;
    $name = $req->param('name');
    $value = $req->param('value');
    ${$name} = $value;
    return BenchmarkResponse::json(['db' => $config_db, 'debug' => $config_debug]);
}
