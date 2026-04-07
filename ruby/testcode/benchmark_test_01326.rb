require_relative 'shared'

def create_user_account(req)
  db = get_db_connection
  name = req.post('name')
  email = req.post('email')
  db.execute("INSERT INTO users (name, email) VALUES (?, ?)", [name, email])
  BenchmarkResponse.ok('account created')
end
