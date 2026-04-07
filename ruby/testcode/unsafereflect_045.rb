require_relative 'shared'

CLASS_MAP2 = { 'admin' => String, 'user' => Integer }.freeze
APP_SERVICES = [String, Integer].freeze

# vuln-code-snippet start ruby_reflect_interface_check
def handler(req)
  klass = CLASS_MAP2.fetch(req.param('type'))
  raise ArgumentError, 'service not registered' unless APP_SERVICES.include?(klass) # vuln-code-snippet safe-line ruby_reflect_interface_check
  instance = klass.new
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_reflect_interface_check
