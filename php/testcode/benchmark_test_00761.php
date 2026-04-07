<?php
require_once __DIR__ . '/shared.php';

class Config011 {
    public static string $db_host = 'localhost';
    public static string $db_name = 'app';
}

function benchmarkTest00761(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('prop');
    $value = $req->param('value');
    Config011::$$prop = $value;
    return BenchmarkResponse::json(['host' => Config011::$db_host, 'db' => Config011::$db_name]);
}
