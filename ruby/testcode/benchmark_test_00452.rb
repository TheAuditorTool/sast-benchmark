require_relative 'shared'

def destroy_post(req)
  post_id = req.param('id').to_i
  db = get_db_connection
  db.execute("DELETE FROM posts WHERE id = ?", [post_id])
  BenchmarkResponse.json({ result: 'deleted' })
end
