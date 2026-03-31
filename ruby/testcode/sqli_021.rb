require_relative 'shared'

def sanitize_search_term(raw)
  raw.strip.gsub(/[^a-zA-Z0-9\s\-_]/, '')
end

def build_search_placeholder(term)
  "%#{term}%"
end

# vuln-code-snippet start ruby_sqli_multihop_sanitized
def search_articles(req)
  db = get_db_connection
  raw_input = req.param('q')
  sanitized = sanitize_search_term(raw_input)
  pattern = build_search_placeholder(sanitized)
  rows = db.execute("SELECT id, title, author, published_at FROM articles WHERE title LIKE ? AND published = 1", [pattern])  # vuln-code-snippet safe-line ruby_sqli_multihop_sanitized
  articles = rows.map { |r| { id: r[0], title: r[1], author: r[2], published_at: r[3] } }
  BenchmarkResponse.json({ results: articles, count: articles.length })
end
# vuln-code-snippet end ruby_sqli_multihop_sanitized
