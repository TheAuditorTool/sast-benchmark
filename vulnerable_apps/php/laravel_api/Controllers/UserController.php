<?php
// laravel_api - UserController: XSS, open redirect, type juggling tests
require_once __DIR__ . '/../../../testcode/shared.php';

// vuln-code-snippet start la_xss_blade_raw
function la_xss_blade_raw(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    $html = '<div class="profile"><h1>Welcome, ' . $name . '!</h1></div>'; // vuln-code-snippet vuln-line la_xss_blade_raw
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end la_xss_blade_raw

// vuln-code-snippet start la_xss_blade_escaped
function la_xss_blade_escaped(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    $safe = htmlspecialchars($name, ENT_QUOTES, 'UTF-8'); // vuln-code-snippet safe-line la_xss_blade_escaped
    $html = '<div class="profile"><h1>Welcome, ' . $safe . '!</h1></div>';
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end la_xss_blade_escaped

// vuln-code-snippet start la_redirect_open
function la_redirect_open(BenchmarkRequest $req): BenchmarkResponse {
    $next = $req->param('next');
    return BenchmarkResponse::redirect($next); // vuln-code-snippet vuln-line la_redirect_open
}
// vuln-code-snippet end la_redirect_open

// vuln-code-snippet start la_redirect_route
function la_redirect_route(BenchmarkRequest $req): BenchmarkResponse {
    return BenchmarkResponse::redirect('/dashboard'); // vuln-code-snippet safe-line la_redirect_route
}
// vuln-code-snippet end la_redirect_route

// vuln-code-snippet start la_tj_api_key_loose
function la_tj_api_key_loose(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->header('X-Api-Key');
    $expected = getenv('API_SECRET_KEY');
    if ($key == $expected) { // vuln-code-snippet vuln-line la_tj_api_key_loose
        return BenchmarkResponse::json(['auth' => 'ok']);
    }
    return BenchmarkResponse::error('unauthorized', 401);
}
// vuln-code-snippet end la_tj_api_key_loose

// vuln-code-snippet start la_tj_api_key_strict
function la_tj_api_key_strict(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->header('X-Api-Key');
    $expected = getenv('API_SECRET_KEY');
    if (hash_equals($expected, $key)) { // vuln-code-snippet safe-line la_tj_api_key_strict
        return BenchmarkResponse::json(['auth' => 'ok']);
    }
    return BenchmarkResponse::error('unauthorized', 401);
}
// vuln-code-snippet end la_tj_api_key_strict
