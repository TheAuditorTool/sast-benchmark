<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00848(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $tpl = $req->post('tpl');
    $_SESSION['tpl'] = $tpl;
    $output = null;
    eval($_SESSION['tpl']);
    return BenchmarkResponse::ok((string) $output);
}
