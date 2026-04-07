<?php
// vuln_blog - File upload and download handling
require_once __DIR__ . '/config.php';

// vuln-code-snippet start vb_upload_no_validate
function uploadFileUnsafe(): void {
    $file = $_FILES['attachment'];
    $dest = 'uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet vuln-line vb_upload_no_validate
    echo 'Uploaded to ' . $dest;
}
// vuln-code-snippet end vb_upload_no_validate

// vuln-code-snippet start vb_upload_safe
function uploadFileSafe(): void {
    $file = $_FILES['attachment'];
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $allowed = ['jpg', 'jpeg', 'png', 'gif'];
    if (!in_array($ext, $allowed, true)) {
        http_response_code(400);
        echo 'Invalid file type';
        return;
    }
    $newName = bin2hex(random_bytes(16)) . '.' . $ext;
    $dest = 'uploads/' . $newName;
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet safe-line vb_upload_safe
    echo 'Uploaded to ' . $dest;
}
// vuln-code-snippet end vb_upload_safe

// vuln-code-snippet start vb_pt_download
function downloadFileUnsafe(): void {
    $filename = $_GET['file'];
    readfile('uploads/' . $filename); // vuln-code-snippet vuln-line vb_pt_download
}
// vuln-code-snippet end vb_pt_download

// vuln-code-snippet start vb_pt_download_safe
function downloadFileSafe(): void {
    $filename = basename($_GET['file']); // vuln-code-snippet safe-line vb_pt_download_safe
    $path = 'uploads/' . $filename;
    if (!file_exists($path)) {
        http_response_code(404);
        echo 'File not found';
        return;
    }
    readfile($path);
}
// vuln-code-snippet end vb_pt_download_safe

// vuln-code-snippet start vb_fi_template
function loadTemplateUnsafe(): void {
    $theme = $_GET['theme'];
    include('themes/' . $theme . '/layout.php'); // vuln-code-snippet vuln-line vb_fi_template
}
// vuln-code-snippet end vb_fi_template

// vuln-code-snippet start vb_fi_template_safe
function loadTemplateSafe(): void {
    $theme = $_GET['theme'];
    $allowedThemes = ['default', 'dark', 'minimal'];
    if (!in_array($theme, $allowedThemes, true)) { // vuln-code-snippet safe-line vb_fi_template_safe
        $theme = 'default';
    }
    include('themes/' . $theme . '/layout.php');
}
// vuln-code-snippet end vb_fi_template_safe
