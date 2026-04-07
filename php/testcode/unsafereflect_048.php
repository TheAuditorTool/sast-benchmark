<?php
require_once __DIR__ . '/shared.php';

class DbConnection {}

// vuln-code-snippet start php_reflect_di_typesafe_resolution
function unsafereflect048(BenchmarkRequest $req): BenchmarkResponse {
    $container = new stdClass();
    $container->bind = function(string $iface, string $concrete) {};
    $svc = new UserService(new DbConnection()); // vuln-code-snippet safe-line php_reflect_di_typesafe_resolution
    return BenchmarkResponse::ok('resolved');
}
// vuln-code-snippet end php_reflect_di_typesafe_resolution
