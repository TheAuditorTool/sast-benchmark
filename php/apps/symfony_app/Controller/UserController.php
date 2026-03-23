<?php
// Symfony-style User Controller
require_once __DIR__ . '/../../../testcode/shared.php';

// vuln-code-snippet start sy_sqli_native
function sy_sqli_native(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $role = $req->param('role');
    $result = $pdo->query("SELECT * FROM users WHERE role='" . $role . "'"); // vuln-code-snippet vuln-line sy_sqli_native
    return BenchmarkResponse::json($result->fetchAll());
}
// vuln-code-snippet end sy_sqli_native

// vuln-code-snippet start sy_sqli_native_safe
function sy_sqli_native_safe(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $role = $req->param('role');
    $stmt = $pdo->prepare("SELECT * FROM users WHERE role=?"); // vuln-code-snippet safe-line sy_sqli_native_safe
    $stmt->execute([$role]);
    return BenchmarkResponse::json($stmt->fetchAll());
}
// vuln-code-snippet end sy_sqli_native_safe

// vuln-code-snippet start sy_ssrf_http_client
function sy_ssrf_http_client(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $data = file_get_contents($url); // vuln-code-snippet vuln-line sy_ssrf_http_client
    return BenchmarkResponse::ok($data);
}
// vuln-code-snippet end sy_ssrf_http_client

// vuln-code-snippet start sy_ssrf_base_uri
function sy_ssrf_base_uri(BenchmarkRequest $req): BenchmarkResponse {
    $baseUri = 'https://api.internal.example.com';
    $path = '/users/profile';
    $data = file_get_contents($baseUri . $path); // vuln-code-snippet safe-line sy_ssrf_base_uri
    return BenchmarkResponse::ok($data);
}
// vuln-code-snippet end sy_ssrf_base_uri

// vuln-code-snippet start sy_reflect_controller
function sy_reflect_controller(BenchmarkRequest $req): BenchmarkResponse {
    $controllerClass = $req->param('controller');
    $obj = new $controllerClass($req); // vuln-code-snippet vuln-line sy_reflect_controller
    return BenchmarkResponse::ok(get_class($obj));
}
// vuln-code-snippet end sy_reflect_controller

// vuln-code-snippet start sy_reflect_service
function sy_reflect_service(BenchmarkRequest $req): BenchmarkResponse {
    $services = [
        'mailer' => 'MailerService',
        'logger' => 'LoggerService',
        'cache'  => 'CacheService',
    ];
    $name = $req->param('service');
    $class = $services[$name] ?? null; // vuln-code-snippet safe-line sy_reflect_service
    if ($class === null) {
        return BenchmarkResponse::badRequest('Unknown service');
    }
    return BenchmarkResponse::ok($class);
}
// vuln-code-snippet end sy_reflect_service
