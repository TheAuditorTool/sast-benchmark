require_relative 'shared'

def fetch_item_prepared(req)
  item_id = req.param('id').to_i
  db = get_db_connection
  stmt = db.prepare("SELECT * FROM items WHERE id = ?")
  rows = stmt.execute(item_id)
  BenchmarkResponse.json({ item: rows.first })
end
