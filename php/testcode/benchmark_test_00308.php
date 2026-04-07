<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00308(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');
    if (!ctype_alpha($page)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    $map = ['home' => '/', 'about' => '/about'];
    $url = $map[$page] ?? '/';
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
