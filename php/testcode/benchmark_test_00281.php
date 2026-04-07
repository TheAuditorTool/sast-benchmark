<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00281(BenchmarkRequest $req): BenchmarkResponse {
    $pharPath = $req->param('phar');
    $webroot = '/var/www/html/';
    $phar = new PharData($pharPath);
    $phar->extractTo($webroot);
    return BenchmarkResponse::ok('extracted');
}
