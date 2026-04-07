<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00260(BenchmarkRequest $req): BenchmarkResponse {
    extract($req->postData, EXTR_REFS | EXTR_PREFIX_ALL, 'user');
    $name = $user_name ?? 'unknown';
    $email = $user_email ?? 'unknown';
    return BenchmarkResponse::json(['name' => $name, 'email' => $email]);
}
