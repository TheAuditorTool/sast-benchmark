<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00446(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['https://app.example.com', 'https://admin.example.com'];
    $origin = $req->header('Origin');
    if (!in_array($origin, $allowed, true)) {
        return BenchmarkResponse::badRequest('origin not allowed');
    }
    header('Access-Control-Allow-Origin: ' . $origin);
    performAction($req->bodyStr());
    return BenchmarkResponse::json(['ok' => true]);
}
