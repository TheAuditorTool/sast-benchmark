<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_route_lookup
function redirect015(BenchmarkRequest $req): BenchmarkResponse {
    $routes = [
        'dashboard' => '/app/dashboard',
        'profile' => '/app/profile',
        'settings' => '/app/settings',
        'logout' => '/auth/logout',
    ];
    $name = $req->param('route');
    if (!isset($routes[$name])) { // vuln-code-snippet safe-line php_redirect_route_lookup
        return BenchmarkResponse::badRequest('Unknown route');
    }
    header("Location: " . $routes[$name]);
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_route_lookup
