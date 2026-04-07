<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00688(BenchmarkRequest $req): BenchmarkResponse {
    $pref = $req->param('theme');
    setcookie('pref', $pref, time() + 86400, '/', '', false, false);
    return BenchmarkResponse::ok('pref saved');
}
