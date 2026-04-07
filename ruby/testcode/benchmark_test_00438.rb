require_relative 'shared'

def upload_and_record(req)
  upload   = req.file('file')
  user_id  = req.post('user_id')
  return BenchmarkResponse.bad_request('no file') unless upload

  filename = upload[:filename]
  dest     = "/var/uploads/#{filename}"
  File.write(dest, upload[:data])

  db = get_db_connection
  db.execute("INSERT INTO uploads (user_id, filename) VALUES (#{user_id}, '#{filename}')")

  BenchmarkResponse.ok('recorded')
end
