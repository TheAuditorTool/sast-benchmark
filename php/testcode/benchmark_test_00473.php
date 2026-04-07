<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00473(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('page');
    $routes = ['home' => '/', 'dashboard' => '/dashboard', 'profile' => '/profile'];
    $url = $routes[$name] ?? '/';
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
