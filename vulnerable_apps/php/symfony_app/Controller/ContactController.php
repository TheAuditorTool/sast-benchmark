<?php
// Symfony-style Contact Controller
require_once __DIR__ . '/../../../testcode/shared.php';

// vuln-code-snippet start sy_sqli_dql_concat
function sy_sqli_dql_concat(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $email = $req->param('email');
    $dql = "SELECT u FROM User u WHERE u.email='" . $email . "'"; // vuln-code-snippet vuln-line sy_sqli_dql_concat
    $result = $pdo->query($dql);
    return BenchmarkResponse::json($result->fetchAll());
}
// vuln-code-snippet end sy_sqli_dql_concat

// vuln-code-snippet start sy_sqli_dql_param
function sy_sqli_dql_param(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $email = $req->param('email');
    $stmt = $pdo->prepare("SELECT * FROM users WHERE email = ?"); // vuln-code-snippet safe-line sy_sqli_dql_param
    $stmt->execute([$email]);
    return BenchmarkResponse::json($stmt->fetchAll());
}
// vuln-code-snippet end sy_sqli_dql_param

// vuln-code-snippet start sy_xss_twig_raw
function sy_xss_twig_raw(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('message');
    $html = '<div class="flash">' . $input . '</div>'; // vuln-code-snippet vuln-line sy_xss_twig_raw
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end sy_xss_twig_raw

// vuln-code-snippet start sy_xss_twig_escaped
function sy_xss_twig_escaped(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('message');
    $html = '<div class="flash">' . htmlspecialchars($input, ENT_QUOTES, 'UTF-8') . '</div>'; // vuln-code-snippet safe-line sy_xss_twig_escaped
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end sy_xss_twig_escaped

// vuln-code-snippet start sy_redirect_open
function sy_redirect_open(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('redirect');
    return new BenchmarkResponse(302, '', ['Location' => $url]); // vuln-code-snippet vuln-line sy_redirect_open
}
// vuln-code-snippet end sy_redirect_open

// vuln-code-snippet start sy_redirect_route
function sy_redirect_route(BenchmarkRequest $req): BenchmarkResponse {
    return new BenchmarkResponse(302, '', ['Location' => '/contact/success']); // vuln-code-snippet safe-line sy_redirect_route
}
// vuln-code-snippet end sy_redirect_route
