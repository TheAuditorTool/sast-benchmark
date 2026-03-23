<?php
// vuln_blog - Post CRUD operations
require_once __DIR__ . '/config.php';

// vuln-code-snippet start vb_sqli_search
function searchPostsUnsafe(string $search): array {
    $pdo = getDbConnectionHardcoded();
    $query = "SELECT * FROM posts WHERE title LIKE '%" . $search . "%'"; // vuln-code-snippet vuln-line vb_sqli_search
    $result = $pdo->query($query);
    return $result->fetchAll(PDO::FETCH_ASSOC);
}
// vuln-code-snippet end vb_sqli_search

// vuln-code-snippet start vb_sqli_search_safe
function searchPostsSafe(string $search): array {
    $pdo = getDbConnectionEnv();
    $stmt = $pdo->prepare("SELECT * FROM posts WHERE title LIKE ?"); // vuln-code-snippet safe-line vb_sqli_search_safe
    $stmt->execute(['%' . $search . '%']);
    return $stmt->fetchAll(PDO::FETCH_ASSOC);
}
// vuln-code-snippet end vb_sqli_search_safe

// vuln-code-snippet start vb_xss_post_title
function renderPostTitleUnsafe(array $row): void {
    echo '<h2>' . $row['title'] . '</h2>'; // vuln-code-snippet vuln-line vb_xss_post_title
    echo '<p>' . $row['body'] . '</p>';
}
// vuln-code-snippet end vb_xss_post_title

// vuln-code-snippet start vb_xss_post_escaped
function renderPostTitleSafe(array $row): void {
    echo '<h2>' . htmlspecialchars($row['title'], ENT_QUOTES, 'UTF-8') . '</h2>'; // vuln-code-snippet safe-line vb_xss_post_escaped
    echo '<p>' . htmlspecialchars($row['body'], ENT_QUOTES, 'UTF-8') . '</p>';
}
// vuln-code-snippet end vb_xss_post_escaped

// vuln-code-snippet start vb_sqli_delete
function deletePostUnsafe(string $id): void {
    $pdo = getDbConnectionHardcoded();
    $pdo->exec("DELETE FROM posts WHERE id=" . $id); // vuln-code-snippet vuln-line vb_sqli_delete
}
// vuln-code-snippet end vb_sqli_delete

// vuln-code-snippet start vb_sqli_delete_safe
function deletePostSafe(string $id): void {
    $pdo = getDbConnectionEnv();
    $stmt = $pdo->prepare("DELETE FROM posts WHERE id = ?"); // vuln-code-snippet safe-line vb_sqli_delete_safe
    $stmt->execute([$id]);
}
// vuln-code-snippet end vb_sqli_delete_safe
