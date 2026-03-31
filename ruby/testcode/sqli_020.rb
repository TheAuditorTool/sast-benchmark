require_relative 'shared'

def normalize_search_term(raw)
  raw.strip.downcase.gsub(/\s+/, ' ')
end

def build_name_filter(term)
  "name LIKE '%#{term}%'"
end

# vuln-code-snippet start ruby_sqli_multihop_taint
def search_products(req)
  db = get_db_connection
  raw_input = req.param('q')
  processed = normalize_search_term(raw_input)
  filter = build_name_filter(processed)
  rows = db.execute("SELECT id, name, price, sku FROM products WHERE #{filter} AND active = 1")  # vuln-code-snippet vuln-line ruby_sqli_multihop_taint
  products = rows.map { |r| { id: r[0], name: r[1], price: r[2], sku: r[3] } }
  BenchmarkResponse.json({ results: products, query: raw_input })
end
# vuln-code-snippet end ruby_sqli_multihop_taint
