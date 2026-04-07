<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_deploy_config
function extract041(BenchmarkRequest $req): BenchmarkResponse {
    $configFile = parse_ini_file('/etc/app/config.ini', false);
    extract($configFile); // vuln-code-snippet safe-line php_extract_deploy_config
    return BenchmarkResponse::ok("db_host=$db_host");
}
// vuln-code-snippet end php_extract_deploy_config
