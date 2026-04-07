require_relative 'shared'

# vuln-code-snippet start ruby_fileupload_path_stored
def upload_and_record(req)
  upload   = req.file('file')
  user_id  = req.post('user_id')
  return BenchmarkResponse.bad_request('no file') unless upload

  filename = upload[:filename]
  dest     = "/var/uploads/#{filename}"
  File.write(dest, upload[:data])

  db = get_db_connection
  db.execute("INSERT INTO uploads (user_id, filename) VALUES (#{user_id}, '#{filename}')") # vuln-code-snippet vuln-line ruby_fileupload_path_stored

  BenchmarkResponse.ok('recorded')
end
# vuln-code-snippet end ruby_fileupload_path_stored
