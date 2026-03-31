require_relative 'shared'

# vuln-code-snippet start ruby_sqli_order_allowlist
ALLOWED_SORT_COLUMNS = %w[name price created_at stock].freeze
ALLOWED_DIRECTIONS   = %w[ASC DESC].freeze

def list_products_sorted(req)
  db = get_db_connection
  column    = req.param('sort', default: 'name')
  direction = req.param('dir',  default: 'ASC').upcase
  column    = 'name'  unless ALLOWED_SORT_COLUMNS.include?(column)
  direction = 'ASC'   unless ALLOWED_DIRECTIONS.include?(direction)
  rows = db.execute("SELECT id, name, price, stock FROM products ORDER BY #{column} #{direction}")  # vuln-code-snippet safe-line ruby_sqli_order_allowlist
  BenchmarkResponse.json({ products: rows, sort: column, direction: direction })
end
# vuln-code-snippet end ruby_sqli_order_allowlist
