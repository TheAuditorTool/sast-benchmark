<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_fixed_path_overwrite
function fileupload031(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    move_uploaded_file($file['tmp_name'], '/var/app/uploads/profile.jpg'); // vuln-code-snippet vuln-line php_upload_fixed_path_overwrite
    return BenchmarkResponse::ok('Profile photo updated');
}
// vuln-code-snippet end php_upload_fixed_path_overwrite
