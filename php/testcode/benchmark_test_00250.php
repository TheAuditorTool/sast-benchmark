<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00250(BenchmarkRequest $req): BenchmarkResponse {
    $isAdmin = false;
    foreach ($_POST as $k => $v) {
        $$k = $v;
    }
    return BenchmarkResponse::ok('prefs set admin=' . ($isAdmin ? '1' : '0'));
}
