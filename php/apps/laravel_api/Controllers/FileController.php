<?php
// laravel_api - FileController: SSRF, path traversal, file upload tests
require_once __DIR__ . '/../../../testcode/shared.php';

// vuln-code-snippet start la_ssrf_webhook
function la_ssrf_webhook(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('callback_url');
    $ch = curl_init($url); // vuln-code-snippet vuln-line la_ssrf_webhook
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    curl_setopt($ch, CURLOPT_POST, true);
    curl_setopt($ch, CURLOPT_POSTFIELDS, json_encode(['event' => 'task.completed']));
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::json(['delivered' => true, 'response' => $result]);
}
// vuln-code-snippet end la_ssrf_webhook

// vuln-code-snippet start la_ssrf_allowlist
function la_ssrf_allowlist(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('callback_url');
    $allowed = ['hooks.slack.com', 'api.github.com', 'notify.example.com'];
    $host = parse_url($url, PHP_URL_HOST);
    if (!in_array($host, $allowed, true)) { // vuln-code-snippet safe-line la_ssrf_allowlist
        return BenchmarkResponse::error('domain not allowed', 403);
    }
    $ch = curl_init($url);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    curl_setopt($ch, CURLOPT_POST, true);
    curl_setopt($ch, CURLOPT_POSTFIELDS, json_encode(['event' => 'task.completed']));
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::json(['delivered' => true, 'response' => $result]);
}
// vuln-code-snippet end la_ssrf_allowlist

// vuln-code-snippet start la_pt_download
function la_pt_download(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    $content = file_get_contents("storage/" . $path); // vuln-code-snippet vuln-line la_pt_download
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end la_pt_download

// vuln-code-snippet start la_pt_download_safe
function la_pt_download_safe(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    $safe = basename($path); // vuln-code-snippet safe-line la_pt_download_safe
    $fullPath = realpath("storage/" . $safe);
    $storageDir = realpath("storage");
    if ($fullPath === false || strpos($fullPath, $storageDir) !== 0) {
        return BenchmarkResponse::error('file not found', 404);
    }
    $content = file_get_contents($fullPath);
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end la_pt_download_safe

// vuln-code-snippet start la_upload_no_validate
function la_upload_no_validate(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('attachment');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    $dest = "storage/uploads/" . $file['name']; // vuln-code-snippet vuln-line la_upload_no_validate
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::json(['path' => $dest]);
}
// vuln-code-snippet end la_upload_no_validate

// vuln-code-snippet start la_upload_validated
function la_upload_validated(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('attachment');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    $allowed = ['jpg', 'jpeg', 'png', 'gif', 'pdf'];
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    if (!in_array($ext, $allowed, true)) { // vuln-code-snippet safe-line la_upload_validated
        return BenchmarkResponse::badRequest('file type not allowed');
    }
    $safeName = bin2hex(random_bytes(16)) . '.' . $ext;
    $dest = "storage/uploads/" . $safeName;
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::json(['path' => $dest]);
}
// vuln-code-snippet end la_upload_validated
