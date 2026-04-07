<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00764(BenchmarkRequest $req): BenchmarkResponse {
    putenv('GOOGLE_APPLICATION_CREDENTIALS=' . getenv('GCP_CREDS_PATH'));
    $project = $req->param('project');
    $scope = 'https://www.googleapis.com/auth/cloud-platform';
    return BenchmarkResponse::ok('gcp auth configured for project: ' . $project);
}
