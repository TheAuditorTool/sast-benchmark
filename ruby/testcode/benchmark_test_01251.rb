require_relative 'shared'

def read_validated_file(req)
  filename = req.param('file').gsub(/[^a-z0-9._\-]/i, '')
  validated_path = File.join('/app/uploads', filename)
  content = IO.popen(['cat', '--', validated_path]) { |io| io.read }
  BenchmarkResponse.ok(content)
end
