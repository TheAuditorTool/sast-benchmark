<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01197(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');
    include('/var/www/html/pages/' . $page . '.php');
    return BenchmarkResponse::ok('rendered');
}
