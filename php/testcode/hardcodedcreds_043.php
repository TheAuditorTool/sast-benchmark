<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_gcp_creds_path
function hardcodedcreds043(BenchmarkRequest $req): BenchmarkResponse {
    putenv('GOOGLE_APPLICATION_CREDENTIALS=' . getenv('GCP_CREDS_PATH')); // vuln-code-snippet safe-line php_hardcoded_gcp_creds_path
    $project = $req->param('project');
    $scope = 'https://www.googleapis.com/auth/cloud-platform';
    return BenchmarkResponse::ok('gcp auth configured for project: ' . $project);
}
// vuln-code-snippet end php_hardcoded_gcp_creds_path
