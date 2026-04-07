<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00073(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');
    $templateDir = __DIR__ . '/templates/';
    ini_set('open_basedir', $templateDir);
    include $templateDir . $page . '.php';
    return BenchmarkResponse::ok('Page rendered');
}
