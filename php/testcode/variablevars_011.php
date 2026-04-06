<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_class_static
class Config011 {
    public static string $db_host = 'localhost';
    public static string $db_name = 'app';
}

function variablevars011(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('prop');
    $value = $req->param('value');
    Config011::$$prop = $value; // vuln-code-snippet vuln-line php_vv_class_static
    return BenchmarkResponse::json(['host' => Config011::$db_host, 'db' => Config011::$db_name]);
}
// vuln-code-snippet end php_vv_class_static
