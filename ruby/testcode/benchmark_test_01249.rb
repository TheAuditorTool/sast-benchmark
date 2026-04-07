require_relative 'shared'

FORMATTERS = {
  'currency' => ->(v) { "$#{format('%.2f', v.to_f)}" },
  'percent'  => ->(v) { "#{format('%.1f', v.to_f)}%" },
  'integer'  => ->(v) { v.to_i.to_s }
}.freeze

def format_value(req)
  key = req.param('formatter')
  value = req.param('value')
  proc_obj = FORMATTERS.fetch(key, FORMATTERS['integer'])
  result = proc_obj.call(value)
  BenchmarkResponse.ok(result)
end
