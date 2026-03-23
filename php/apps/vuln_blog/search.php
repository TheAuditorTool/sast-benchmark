<?php
// vuln_blog - Search functionality and redirects
require_once __DIR__ . '/config.php';
require_once __DIR__ . '/posts.php';

// vuln-code-snippet start vb_xss_search_reflect
function renderSearchResultsUnsafe(): void {
    $query = $_GET['q'];
    $results = searchPostsSafe($query);
    echo '<h1>Search results for: ' . $query . '</h1>'; // vuln-code-snippet vuln-line vb_xss_search_reflect
    foreach ($results as $row) {
        echo '<div>' . htmlspecialchars($row['title'], ENT_QUOTES, 'UTF-8') . '</div>';
    }
}
// vuln-code-snippet end vb_xss_search_reflect

// vuln-code-snippet start vb_xss_search_escaped
function renderSearchResultsSafe(): void {
    $query = $_GET['q'];
    $results = searchPostsSafe($query);
    echo '<h1>Search results for: ' . htmlspecialchars($query, ENT_QUOTES, 'UTF-8') . '</h1>'; // vuln-code-snippet safe-line vb_xss_search_escaped
    foreach ($results as $row) {
        echo '<div>' . htmlspecialchars($row['title'], ENT_QUOTES, 'UTF-8') . '</div>';
    }
}
// vuln-code-snippet end vb_xss_search_escaped

// vuln-code-snippet start vb_headerinj_redirect
function redirectAfterLoginUnsafe(): void {
    $url = $_GET['redirect'];
    header('Location: ' . $url); // vuln-code-snippet vuln-line vb_headerinj_redirect
}
// vuln-code-snippet end vb_headerinj_redirect

// vuln-code-snippet start vb_headerinj_safe
function redirectAfterLoginSafe(): void {
    $url = $_GET['redirect'];
    $url = str_replace(["\r", "\n"], '', $url); // vuln-code-snippet safe-line vb_headerinj_safe
    header('Location: ' . $url);
}
// vuln-code-snippet end vb_headerinj_safe
