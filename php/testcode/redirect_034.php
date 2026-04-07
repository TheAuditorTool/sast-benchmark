<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_route_name_lookup
function redirect034(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('page');
    $routes = ['home' => '/', 'dashboard' => '/dashboard', 'profile' => '/profile'];
    $url = $routes[$name] ?? '/'; // vuln-code-snippet safe-line php_redirect_route_name_lookup
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_route_name_lookup
