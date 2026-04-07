<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00637(BenchmarkRequest $req): BenchmarkResponse {
    $isAdmin = false;
    $role = 'user';
    extract($_POST, EXTR_REFS);
    return BenchmarkResponse::ok("role=$role admin=" . ($isAdmin ? '1' : '0'));
}
