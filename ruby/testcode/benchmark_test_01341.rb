require_relative 'shared'

def generate_invoice_html(req)
  layout = req.post('layout')
  rendered = ERB.new(layout).result(binding)
  BenchmarkResponse.html(rendered)
end
