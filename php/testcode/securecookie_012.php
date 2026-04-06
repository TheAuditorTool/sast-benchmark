<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_path_phpself
function securecookie012(BenchmarkRequest $req): BenchmarkResponse {
    $val = $req->post('theme');
    setcookie('theme', $val, 0, $_SERVER['PHP_SELF']); // vuln-code-snippet vuln-line php_cookie_path_phpself
    return BenchmarkResponse::ok('Theme saved');
}
// vuln-code-snippet end php_cookie_path_phpself
