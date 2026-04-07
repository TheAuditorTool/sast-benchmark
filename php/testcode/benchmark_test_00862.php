<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00862(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['name', 'email', 'phone'];
    $filtered = array_intersect_key($req->postData, array_flip($allowed));
    extract($filtered);
    return BenchmarkResponse::json([
        'name' => $name ?? '',
        'email' => $email ?? '',
        'phone' => $phone ?? '',
    ]);
}
