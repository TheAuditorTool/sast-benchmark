<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00324(BenchmarkRequest $req): BenchmarkResponse {
    $referer = $req->header('Referer');
    if (!empty($referer) && !str_starts_with($referer, 'https://app.example.com/')) {
        return BenchmarkResponse::badRequest('bad referer');
    }
    performDelete($req->param('id'));
    return BenchmarkResponse::ok('deleted');
}
