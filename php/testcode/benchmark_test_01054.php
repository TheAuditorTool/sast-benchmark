<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01054(BenchmarkRequest $req): BenchmarkResponse {
    ini_set('open_basedir', '/var/app/templates');
    $tpl = basename($req->param('tpl'));
    include '/var/app/templates/' . $tpl . '.php';
    return BenchmarkResponse::ok('Rendered');
}
