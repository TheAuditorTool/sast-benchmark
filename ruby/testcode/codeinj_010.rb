require_relative 'shared'

FORMATTERS = {
  'currency' => ->(v) { "$#{format('%.2f', v.to_f)}" },
  'percent'  => ->(v) { "#{format('%.1f', v.to_f)}%" },
  'integer'  => ->(v) { v.to_i.to_s }
}.freeze

# vuln-code-snippet start ruby_codeinj_const_proc
def format_value(req)
  key = req.param('formatter')
  value = req.param('value')
  proc_obj = FORMATTERS.fetch(key, FORMATTERS['integer']) # vuln-code-snippet safe-line ruby_codeinj_const_proc
  result = proc_obj.call(value)
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_codeinj_const_proc
