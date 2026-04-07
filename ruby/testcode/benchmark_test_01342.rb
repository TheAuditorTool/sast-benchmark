require_relative 'shared'

def generate_invoice_html(req)
  company = req.post('company')
  amount = req.post('amount')
  layout = '<html><body><p>Invoice for <%= data[:company] %>: $<%= data[:amount] %></p></body></html>'
  rendered = ERB.new(layout).result_with_hash(data: { company: company, amount: amount })
  BenchmarkResponse.html(rendered)
end
