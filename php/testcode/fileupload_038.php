<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_htaccess_deny_dir
function fileupload038(BenchmarkRequest $req): BenchmarkResponse {
    $uploadDir = '/var/www/uploads/';
    $htaccess  = $uploadDir . '.htaccess';
    if (!file_exists($htaccess)) {
        file_put_contents($htaccess, "deny from all\n"); // vuln-code-snippet safe-line php_upload_htaccess_deny_dir
    }
    $file = $_FILES['f'];
    $dest = $uploadDir . bin2hex(random_bytes(8)) . '.dat';
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
// vuln-code-snippet end php_upload_htaccess_deny_dir
