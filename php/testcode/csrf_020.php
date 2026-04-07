<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_delete_no_token
function csrf020(BenchmarkRequest $req): BenchmarkResponse {
    $resourceId = (int) $req->param('id');
    deleteResource($resourceId); // vuln-code-snippet vuln-line php_csrf_delete_no_token
    return BenchmarkResponse::ok('resource deleted');
}
// vuln-code-snippet end php_csrf_delete_no_token
