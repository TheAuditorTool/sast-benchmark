require_relative 'shared'

BASE_TEMPLATES = '/var/app/templates'

def read_template(req)
  template_name = req.param('template')
  resolved = File.realpath("#{BASE_TEMPLATES}/#{template_name}.html")
  return BenchmarkResponse.bad_request('not found') unless resolved.start_with?(BASE_TEMPLATES)
  BenchmarkResponse.html(File.read(resolved))
end
