<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00450(BenchmarkRequest $req): BenchmarkResponse {
    $routes = [
        'dashboard' => '/app/dashboard',
        'profile' => '/app/profile',
        'settings' => '/app/settings',
        'logout' => '/auth/logout',
    ];
    $name = $req->param('route');
    if (!isset($routes[$name])) {
        return BenchmarkResponse::badRequest('Unknown route');
    }
    header("Location: " . $routes[$name]);
    return BenchmarkResponse::ok('');
}
