<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00504(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('executable');
    $args = explode(" ", $req->param('args'));
    pcntl_exec($path, $args);
    return BenchmarkResponse::error("exec failed");
}
