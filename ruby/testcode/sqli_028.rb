require_relative 'shared'

# vuln-code-snippet start ruby_sqli_union_inject
def search_catalog(req)
  db = get_db_connection
  keyword = req.param('keyword')
  category_id = req.param('category_id', default: '0').to_i
  sql = "SELECT id, title, description, price FROM products WHERE category_id = #{category_id} AND title LIKE '%#{keyword}%'"  # vuln-code-snippet vuln-line ruby_sqli_union_inject
  rows = db.execute(sql)
  items = rows.map { |r| { id: r[0], title: r[1], description: r[2], price: r[3] } }
  BenchmarkResponse.json({ items: items, keyword: keyword })
end
# vuln-code-snippet end ruby_sqli_union_inject
