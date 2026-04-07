<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00647(BenchmarkRequest $req): BenchmarkResponse {
    $allowedNames = ['theme', 'lang', 'timezone'];
    $name = $req->param('name');
    $value = $req->param('value');
    if (in_array($name, $allowedNames, true)) {
        setcookie($name, urlencode($value));
        return BenchmarkResponse::ok('Cookie set');
    }
    return BenchmarkResponse::badRequest('Invalid cookie name');
}
